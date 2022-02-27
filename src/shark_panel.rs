use strum::{EnumCount, EnumIter};

use crate::virpil_device::{ToAxisIndex, ToButtonIndex};
use crate::{BoardType, ToBoardAndLedNumber, VirpilDeviceDescription};

pub struct SharkPanel;
impl VirpilDeviceDescription for SharkPanel {
    type Led = SharkPanelLed;
    type Buttons = SharkPanelButtons;
    type Axis = SharkPanelAxis;
    const PID: u16 = 0x825D;
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum SharkPanelLed {
    B1 = 8,
    B2 = 9,
    B3 = 7,
    B4 = 10,
    Rst = 1,
    B5 = 2,
    B6 = 3,
    Dir = 4,
    B7 = 6,
    B8 = 5,
}
impl ToBoardAndLedNumber for SharkPanelLed {
    fn to_board_and_led_number(&self) -> (BoardType, u8) {
        (BoardType::OnBoard, *self as u8)
    }
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum SharkPanelButtons {
    B1 = 1,
    B2 = 2,
    Start = 3,
    APU = 4,
    Stop = 5,
    B3 = 6,
    B4 = 7,
    LHEngine = 8,
    RHEngine = 9,
    StartCrankUp = 10,
    StartCrankDown = 11,
    MasterArmUp = 12,
    MasterArmDown = 13,
    HMSUp = 14,
    HMSDown = 15,
    AutoTsUp = 16,
    AutoTsDown = 17,
    LASUp = 18,
    LASDown = 19,
    Rst = 20,
    B5 = 21,
    B6 = 22,
    Dir = 23,
    E1Press = 24,
    E1CounterClockwise = 25,
    E1Clockwise = 26,
    B7 = 27,
    B8 = 28,
    E2Press = 29,
    E2CounterClockwise = 30,
    E2Clockwise = 31,
    ModeMOV = 32,
    ModeFIX = 33,
    ModeMan = 34,
    ModeFail = 35,
    ModeNav = 36,
    ManUp = 37,
    ManDown = 38,
    RangeLng = 39,
    RangeMd = 40,
    RangeShort = 41,
    HE = 42,
    API = 43,
    Low = 44,
    High = 45,
}
impl ToButtonIndex for SharkPanelButtons {
    fn to_button_index(&self) -> u8 {
        *self as u8 - 1
    }
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum SharkPanelAxis {
    RightOuter = 1,
    RightInner = 2,
    LeftInner = 3,
    LeftOuter = 4,
}
impl ToAxisIndex for SharkPanelAxis {
    fn to_axis_index(&self) -> u8 {
        *self as u8 - 1
    }
}
