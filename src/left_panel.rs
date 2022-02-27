use strum::{EnumCount, EnumIter};

use crate::virpil_device::{ToAxisIndex, ToButtonIndex, VirpilDeviceDescription};
use crate::{BoardType, ToBoardAndLedNumber};

#[derive(Debug, Copy, Clone)]
pub struct LeftPanel;
impl VirpilDeviceDescription for LeftPanel {
    type Led = LeftPanelLed;
    type Buttons = LeftPanelButtons;
    type Axis = LeftPanelAxis;
    const PID: u16 = 0x025B;
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum LeftPanelLed {
    B1 = 2,
    B2 = 1,
    B3 = 4,
    B4 = 3,
    B5 = 17,
    B6 = 14,
    B7 = 16,
    B8 = 13,
    B9 = 15,
    B10 = 12,
    Airbrake = 5,
    Warning = 6,
    FlapLeft = 7,
    FlapRight = 11,
    GearLeft = 8,
    GearCenter = 9,
    GearRight = 10,
}
impl ToBoardAndLedNumber for LeftPanelLed {
    fn to_board_and_led_number(&self) -> (BoardType, u8) {
        (BoardType::OnBoard, *self as u8)
    }
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum LeftPanelButtons {
    B1 = 1,
    B2 = 2,
    B3 = 3,
    B4 = 4,
    B5 = 5,
    B6 = 6,
    B7 = 7,
    B8 = 8,
    B9 = 9,
    B10 = 10,
    T1Up = 11,
    T1Down = 12,
    T2Up = 13,
    T2Down = 14,
    T3Up = 15,
    T3Down = 16,
    T4Up = 17,
    T4Down = 18,
    T5Up = 19,
    T5Down = 20,
    T6Up = 21,
    T6Down = 22,
    T7Guard = 23,
    T7 = 24,
    T8Guard = 25,
    T8 = 26,
    T9Left = 27,
    T9Right = 28,
    T10Left = 29,
    T10Right = 30,
    E1Press = 31,
    E1CounterClockwise = 32,
    E1Clockwise = 33,
    E2Press = 34,
    E2CounterClockwise = 35,
    E2Clockwise = 36,
    E3Press = 37,
    E3CounterClockwise = 38,
    E3Clockwise = 39,
    GearMiddle = 40,
    GearUp = 41,
    GearDown = 42,
}
impl ToButtonIndex for LeftPanelButtons {
    fn to_button_index(&self) -> u8 {
        *self as u8 - 1
    }
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum LeftPanelAxis {}
impl ToAxisIndex for LeftPanelAxis {
    fn to_axis_index(&self) -> u8 {
        unreachable!()
    }
}
