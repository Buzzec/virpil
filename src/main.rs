#![allow(incomplete_features)]
#![feature(generic_const_exprs, split_array)]

use std::io::{stdin, Read};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant, SystemTime};

use ctrlc::set_handler;
use hidapi::{HidApi, HidDevice, HidResult};
use strum::IntoEnumIterator;

use crate::left_panel::{LeftPanel, LeftPanelButtons, LeftPanelLed};
use crate::right_panel::{RightPanel, RightPanelLed};
use crate::right_stick::{RightStick, RightStickLed};
use crate::shark_panel::{SharkPanel, SharkPanelLed};
use crate::throttle::{Throttle, ThrottleLed};
use crate::virpil_device::{find_device, VirpilDevice, VirpilDeviceDescription};

pub mod left_panel;
pub mod right_panel;
pub mod right_stick;
pub mod shark_panel;
pub mod throttle;
pub mod virpil_device;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BoardType {
    Default = 0x64,
    AddBoard = 0x65,
    OnBoard = 0x66,
    SlaveBoard = 0x67,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LedPower {
    /// Off
    Zero = 0,
    /// Really 25%
    Thirty = 1,
    /// Really 50%
    Sixty = 2,
    /// 100%
    Full = 3,
}
impl LedPower {
    pub const OFF: Color = [LedPower::Zero, LedPower::Zero, LedPower::Zero];
    pub const FULL_RED: Color = [LedPower::Full, LedPower::Zero, LedPower::Zero];
    pub const FULL_YELLOW: Color = [LedPower::Full, LedPower::Full, LedPower::Zero];
    pub const FULL_GREEN: Color = [LedPower::Zero, LedPower::Full, LedPower::Zero];
    pub const FULL_CYAN: Color = [LedPower::Zero, LedPower::Full, LedPower::Full];
    pub const FULL_BLUE: Color = [LedPower::Zero, LedPower::Zero, LedPower::Full];
    pub const FULL_MAGENTA: Color = [LedPower::Full, LedPower::Zero, LedPower::Full];
    pub const FULL_WHITE: Color = [LedPower::Full, LedPower::Full, LedPower::Full];

    pub const DEFAULT_RED: Color = [LedPower::Sixty, LedPower::Zero, LedPower::Zero];

    pub const COLOR_PROGRESSION: &'static [Color] = &[
        Self::FULL_RED,
        [LedPower::Full, LedPower::Thirty, LedPower::Zero],
        [LedPower::Full, LedPower::Sixty, LedPower::Zero],
        Self::FULL_YELLOW,
        [LedPower::Sixty, LedPower::Full, LedPower::Zero],
        [LedPower::Thirty, LedPower::Full, LedPower::Zero],
        Self::FULL_GREEN,
        [LedPower::Zero, LedPower::Full, LedPower::Thirty],
        [LedPower::Zero, LedPower::Full, LedPower::Sixty],
        Self::FULL_CYAN,
        [LedPower::Zero, LedPower::Sixty, LedPower::Full],
        [LedPower::Zero, LedPower::Thirty, LedPower::Full],
        Self::FULL_BLUE,
        [LedPower::Thirty, LedPower::Zero, LedPower::Full],
        [LedPower::Sixty, LedPower::Zero, LedPower::Full],
        Self::FULL_MAGENTA,
        [LedPower::Full, LedPower::Zero, LedPower::Sixty],
        [LedPower::Full, LedPower::Zero, LedPower::Thirty],
    ];
}

pub type Color = [LedPower; 3];

pub trait ToBoardAndLedNumber {
    fn to_board_and_led_number(&self) -> (BoardType, u8);
}

fn color_to_byte(color: [LedPower; 3]) -> u8 {
    0b_1000_0000 | color[0] as u8 | (color[1] as u8) << 2 | (color[2] as u8) << 4
}

fn command_id_for_command(board_type: BoardType, led_number: u8) -> u8 {
    match board_type {
        BoardType::Default => 0,
        BoardType::AddBoard => led_number,
        BoardType::OnBoard => led_number + 4,
        BoardType::SlaveBoard => led_number + 24,
    }
}

fn packet_for_command(board_type: BoardType, led_number: u8, color: [LedPower; 3]) -> [u8; 38] {
    let mut out = [0; 38];
    out[0] = 0x02;
    out[1] = board_type as u8;
    out[2] = command_id_for_command(board_type, led_number);
    out[led_number as usize + 4] = color_to_byte(color);
    out[37] = 0xf0;
    out
}

fn send_command(
    device: &HidDevice,
    board_type: BoardType,
    led_number: u8,
    color: [LedPower; 3],
) -> HidResult<()> {
    let packet = packet_for_command(board_type, led_number, color);

    device.send_feature_report(&packet)?;
    Ok(())
}

fn main() {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = stop.clone();
    set_handler(move || {
        stop_clone.store(true, Ordering::Relaxed);
    })
    .unwrap();

    let hid = HidApi::new().unwrap();

    let mut shark_panel = find_device::<SharkPanel>(&hid, LedPower::FULL_RED).unwrap();
    let mut throttle = find_device::<Throttle>(&hid, LedPower::FULL_RED).unwrap();
    let mut left_panel = find_device::<LeftPanel>(&hid, LedPower::FULL_RED).unwrap();
    let mut right_panel = find_device::<RightPanel>(&hid, LedPower::FULL_RED).unwrap();
    let mut right_stick = find_device::<RightStick>(&hid, LedPower::FULL_RED).unwrap();

    type Device = SharkPanel;
    let device: &mut VirpilDevice<Device> = &mut shark_panel;
    let mut color_index = 0;
    const TIME: Duration = Duration::from_millis(500);
    while !stop.load(Ordering::Relaxed) {
        color_index = (color_index + 1) % LedPower::COLOR_PROGRESSION.len();
        let color = LedPower::COLOR_PROGRESSION[color_index];

        // for led in <<Device as VirpilDeviceDescription>::Led as IntoEnumIterator>::iter() {
        //     println!("Set {:?}", led);
        //     let _ = stdin().read(&mut [0; 128]).unwrap();
        //     device.set_led(led, color).unwrap();
        //     sleep(Duration::from_millis(100));
        // }

        let mut times = [Instant::now(); 6];
        let mut times_ref = times.iter_mut();

        *times_ref.next().unwrap() = Instant::now();
        for led in SharkPanelLed::iter() {
            if let Some(err) = shark_panel.set_led(led, color).err() {
                eprintln!("Error encountered: {}", err)
            }
        }
        *times_ref.next().unwrap() = Instant::now();
        for led in ThrottleLed::iter() {
            if let Some(err) = throttle.set_led(led, color).err() {
                eprintln!("Error encountered: {}", err)
            }
        }
        *times_ref.next().unwrap() = Instant::now();
        for led in LeftPanelLed::iter() {
            if let Some(err) = left_panel.set_led(led, color).err() {
                eprintln!("Error encountered: {}", err)
            }
        }
        *times_ref.next().unwrap() = Instant::now();
        for led in RightPanelLed::iter() {
            if let Some(err) = right_panel.set_led(led, color).err() {
                eprintln!("Error encountered: {}", err)
            }
        }
        *times_ref.next().unwrap() = Instant::now();
        for led in RightStickLed::iter() {
            if let Some(err) = right_stick.set_led(led, color).err() {
                eprintln!("Error encountered: {}", err)
            }
        }
        *times_ref.next().unwrap() = Instant::now();

        drop(times_ref);
        for index in 0..times.len() - 1 {
            println!(
                "Time {}: {:?}",
                index,
                times[index + 1].duration_since(times[index])
            );
        }
        println!(
            "Total:{:?}",
            times[times.len() - 1].duration_since(times[0])
        );
        sleep(TIME);
        while shark_panel.send_queue_size() > 0 {}
        while throttle.send_queue_size() > 0 {}
        while left_panel.send_queue_size() > 0 {}
        while right_panel.send_queue_size() > 0 {}
        while right_stick.send_queue_size() > 0 {}
    }

    // left.join().unwrap();

    // let control = virpil_devices
    //     .iter()
    //     .find(|dev| dev.product_id() == PANEL_1_PID[0] && dev.usage() == 4)
    //     .unwrap()
    //     .open_device(&hid)
    //     .unwrap();
    // let mut buf = [0; 128];
    // // buf[0] = 2;
    // // let read = other_throttle.get_feature_report(&mut buf).unwrap();
    // // println!("{:?}", &buf[0..read]);
    // loop {
    //     let read = control.read(&mut buf).unwrap();
    //     println!("{:?}", &buf[0..read]);
    // }
}
