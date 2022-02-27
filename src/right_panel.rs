use strum::{EnumCount, EnumIter};

use crate::virpil_device::{ToAxisIndex, ToButtonIndex, VirpilDeviceDescription};
use crate::{BoardType, ToBoardAndLedNumber};

#[derive(Debug, Copy, Clone)]
pub struct RightPanel;
impl VirpilDeviceDescription for RightPanel {
    type Led = RightPanelLed;
    type Buttons = RightPanelButtons;
    type Axis = RightPanelAxis;

    const PID: u16 = 0x0259;
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum RightPanelLed {
    B1 = 13,
    B2 = 10,
    B3 = 12,
    B4 = 9,
    B5 = 11,
    B6 = 8,
    B7 = 5,
    B8 = 6,
    B9 = 7,
    B10 = 2,
    B11 = 3,
    B12 = 4,
}
impl ToBoardAndLedNumber for RightPanelLed {
    fn to_board_and_led_number(&self) -> (BoardType, u8) {
        (BoardType::OnBoard, *self as u8 - 1)
    }
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum RightPanelAxis {
    A1 = 0,
    A2 = 1,
}
impl ToAxisIndex for RightPanelAxis {
    fn to_axis_index(&self) -> u8 {
        *self as u8
    }
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum RightPanelButtons {
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
    B11 = 11,
    B12 = 12,
    T1Guard = 13,
    T1 = 14,
    T2Guard = 15,
    T2 = 16,
    T3Left = 17,
    T3Right = 18,
    T4Left = 19,
    T4Right = 20,
    T5Down = 21,
    T5Up = 22,
    T6Down = 23,
    T6Up = 24,
    T7Down = 25,
    T7Up = 26,
    T8Down = 27,
    T8Up = 28,
    T9Down = 29,
    T9Up = 30,
    T10Down = 31,
    T10Up = 32,
    E1Press = 33,
    E1CounterClockwise = 34,
    E1Clockwise = 35,
    E2Press = 36,
    E2CounterClockwise = 37,
    E2Clockwise = 38,
    E3Press = 39,
    E3CounterClockwise = 40,
    E3Clockwise = 41,
    A1Left = 42,
    A1Middle = 43,
    A1Right = 44,
    A2Left = 45,
    A2Middle = 46,
    A2Right = 47,
}
impl ToButtonIndex for RightPanelButtons {
    fn to_button_index(&self) -> u8 {
        *self as u8 - 1
    }
}
