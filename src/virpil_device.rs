use core::cmp::Eq;
use core::hash::Hash;
use core::result::Result::Ok;
use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU8, Ordering};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

use array_init::array_init;
use crossbeam::channel::{unbounded, Receiver, Sender};
use hidapi::{HidApi, HidDevice, HidResult};
use strum::{EnumCount, EnumIter, IntoEnumIterator};

use crate::{send_command, BoardType, Color, LedPower, ToBoardAndLedNumber};

pub const VIRPIL_VID: u16 = 0x3344;

pub const MAX_AXIS_VALUE: u16 = u16::from_le_bytes([0, 64]);

pub trait VirpilDeviceDescription {
    type Led: ToBoardAndLedNumber + IntoEnumIterator + EnumCount + Eq + Hash + Send + Copy;
    type Buttons: ToButtonIndex + IntoEnumIterator + EnumCount + Eq + Hash + Copy;
    type Axis: ToAxisIndex + IntoEnumIterator + EnumCount + Eq + Hash + Copy;

    const PID: u16;
}
pub trait ToButtonIndex {
    fn to_button_index(&self) -> u8;
}
pub trait ToAxisIndex {
    fn to_axis_index(&self) -> u8;
}

pub fn find_device<D>(hid: &HidApi, starting_color: Color) -> HidResult<VirpilDevice<D>>
where
    D: VirpilDeviceDescription + 'static,
    [(); D::Axis::COUNT]:,
    [(); D::Buttons::COUNT / 8 + (D::Buttons::COUNT % 8 > 0) as usize]:,
{
    let mut led_write = None;
    let mut state_read = None;
    for device in hid.device_list() {
        if device.vendor_id() == VIRPIL_VID && device.product_id() == D::PID {
            match device.usage() {
                4 => assert!(state_read.replace(device.open_device(hid)?).is_none()),
                1 => assert!(led_write.replace(device.open_device(hid)?).is_none()),
                x => panic!("Unknown usage {}", x),
            }
        }
    }
    VirpilDevice::new(state_read.unwrap(), led_write.unwrap(), starting_color)
}

pub struct VirpilDevice<D>
where
    D: VirpilDeviceDescription + 'static,
    [(); D::Axis::COUNT]:,
    [(); D::Buttons::COUNT / 8 + (D::Buttons::COUNT % 8 > 0) as usize]:,
{
    threads: Option<[JoinHandle<()>; 2]>,
    state: Arc<State<D>>,
    led_write: ManuallyDrop<Sender<(D::Led, Color)>>,
    led_states: HashMap<D::Led, Color>,
}
impl<D> VirpilDevice<D>
where
    D: VirpilDeviceDescription + 'static,
    [(); D::Axis::COUNT]:,
    [(); D::Buttons::COUNT / 8 + (D::Buttons::COUNT % 8 > 0) as usize]:,
{
    pub fn new(
        state_read: HidDevice,
        led_write: HidDevice,
        starting_color: Color,
    ) -> HidResult<Self> {
        let mut led_states = HashMap::with_capacity(D::Led::COUNT);
        for val in D::Led::iter() {
            let (board_type, led_number) = val.to_board_and_led_number();
            send_command(&led_write, board_type, led_number, starting_color)?;
            led_states.insert(val, starting_color);
        }
        let state = Arc::<State<D>>::default();
        let state_clone = state.clone();
        let (sender, receiver) = unbounded();
        Ok(Self {
            threads: Some([
                spawn(move || Self::state_read_loop(state_clone, state_read)),
                spawn(move || Self::led_write_loop(led_write, receiver)),
            ]),
            state,
            led_write: ManuallyDrop::new(sender),
            led_states,
        })
    }

    pub fn button_state(&self, button: D::Buttons) -> bool {
        let index = button.to_button_index();
        self.state.buttons[index as usize / 8].load(Ordering::SeqCst) & (1 << (index % 8)) > 0
    }

    pub fn axis_state(&self, axis: D::Axis) -> u16 {
        self.state.axis[axis.to_axis_index() as usize].load(Ordering::SeqCst)
    }

    pub fn axis_percent(&self, axis: D::Axis) -> f32 {
        self.axis_state(axis) as f32 / MAX_AXIS_VALUE as f32
    }

    pub fn set_led(&mut self, led: D::Led, color: Color) -> HidResult<Color> {
        if self.led_states.get(&led).unwrap() != &color {
            self.led_write.send((led, color)).unwrap();
            Ok(self
                .led_states
                .insert(led, color)
                .expect("led state missing enum value!"))
        } else {
            Ok(color)
        }
    }

    pub fn send_queue_size(&self) -> usize {
        self.led_write.len()
    }

    fn state_read_loop(state: Arc<State<D>>, state_read: HidDevice) {
        let mut buffer = [0; 64];
        while !state.stop.load(Ordering::Relaxed) {
            match state_read.read(&mut buffer) {
                Ok(count)
                    if count
                        == 1 + D::Axis::COUNT * 2
                            + D::Buttons::COUNT / 8
                            + (D::Buttons::COUNT % 8 > 0) as usize =>
                {
                    let mut data = &buffer[1..];
                    for axis in state.axis.iter() {
                        let (val, rest) = data.split_array_ref();
                        data = rest;
                        axis.store(u16::from_le_bytes(*val), Ordering::SeqCst);
                    }
                    for button in state.buttons.iter() {
                        let (val, rest) = data.split_array_ref::<1>();
                        data = rest;
                        button.store(val[0], Ordering::SeqCst);
                    }
                }
                Ok(count) => eprintln!(
                    "Weird account data length ({}) from {}: {:?}",
                    count,
                    state_read.get_product_string().unwrap().unwrap(),
                    &buffer[..count]
                ),
                Err(error) => eprintln!(
                    "Error on {} read: {}",
                    state_read.get_product_string().unwrap().unwrap(),
                    error
                ),
            };
        }
    }

    fn led_write_loop(led_write: HidDevice, write_receiver: Receiver<(D::Led, Color)>) {
        while let Ok(command) = write_receiver.recv() {
            let (board_type, led_number) = command.0.to_board_and_led_number();
            if let Err(error) = send_command(&led_write, board_type, led_number, command.1) {
                println!(
                    "Error Setting {} led {} on board {:?} to {:?}! {}",
                    led_write.get_product_string().unwrap().unwrap(),
                    led_number,
                    board_type,
                    command.1,
                    error
                );
            }
        }
    }
}
impl<D> Drop for VirpilDevice<D>
where
    D: VirpilDeviceDescription + 'static,
    [(); D::Axis::COUNT]:,
    [(); D::Buttons::COUNT / 8 + (D::Buttons::COUNT % 8 > 0) as usize]:,
{
    fn drop(&mut self) {
        self.state.stop.store(true, Ordering::Relaxed);
        for led in D::Led::iter() {
            let _ = self.set_led(led, LedPower::DEFAULT_RED);
        }
        unsafe { ManuallyDrop::drop(&mut self.led_write) }
        let handles = self.threads.take().unwrap();
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

pub struct State<D>
where
    D: VirpilDeviceDescription,
    [(); D::Axis::COUNT]:,
    [(); D::Buttons::COUNT / 8 + (D::Buttons::COUNT % 8 > 0) as usize]:,
{
    stop: AtomicBool,
    axis: [AtomicU16; D::Axis::COUNT],
    buttons: [AtomicU8; D::Buttons::COUNT / 8 + (D::Buttons::COUNT % 8 > 0) as usize],
}
impl<D> Default for State<D>
where
    D: VirpilDeviceDescription,
    [(); D::Axis::COUNT]:,
    [(); D::Buttons::COUNT / 8 + (D::Buttons::COUNT % 8 > 0) as usize]:,
{
    fn default() -> Self {
        Self {
            stop: AtomicBool::new(false),
            axis: array_init(|_| AtomicU16::new(0)),
            buttons: array_init(|_| AtomicU8::new(0)),
        }
    }
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum AllOnBoard {
    L1 = 1,
    L2 = 2,
    L3 = 3,
    L4 = 4,
    L5 = 5,
    L6 = 6,
    L7 = 7,
    L8 = 8,
    L9 = 9,
    L10 = 10,
    L11 = 11,
    L12 = 12,
    L13 = 13,
    L14 = 14,
    L15 = 15,
    L16 = 16,
    L17 = 17,
    L18 = 18,
    L19 = 19,
    L20 = 20,
}
impl ToBoardAndLedNumber for AllOnBoard {
    fn to_board_and_led_number(&self) -> (BoardType, u8) {
        (BoardType::OnBoard, *self as u8)
    }
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum AllAddBoard {
    G1 = 1,
    G2 = 2,
    G3 = 3,
    G4 = 4,
}
impl ToBoardAndLedNumber for AllAddBoard {
    fn to_board_and_led_number(&self) -> (BoardType, u8) {
        (BoardType::AddBoard, *self as u8)
    }
}
