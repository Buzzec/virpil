use strum::{EnumCount, EnumIter};

use crate::virpil_device::{ToAxisIndex, ToButtonIndex, VirpilDeviceDescription};
use crate::{BoardType, ToBoardAndLedNumber};

pub struct RightStick;
impl VirpilDeviceDescription for RightStick {
    type Led = RightStickLed;
    type Buttons = RightStickButtons;
    type Axis = RightStickAxis;
    const PID: u16 = 0x4130;
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum RightStickLed {
    Top = 1,
}
impl ToBoardAndLedNumber for RightStickLed {
    fn to_board_and_led_number(&self) -> (BoardType, u8) {
        (BoardType::AddBoard, *self as u8)
    }
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum RightStickButtons {
    GuardOut = 1,
    GuardTap = 2,
    TriggerFirst = 3,
    TriggerSecond = 4,
    ThumbStickPress = 5,
    UpperBlack = 6,
    LowerHatPress = 7,
    LowerHatUp = 8,
    LowerHatRight = 9,
    LowerHatDown = 10,
    LowerHatLeft = 11,
    UpperRed = 12,
    UpperHatPress = 13,
    UpperHatUp = 14,
    UpperHatRight = 15,
    UpperHatDown = 16,
    UpperHatLeft = 17,
    IndexHatPress = 18,
    IndexHatUp = 19,
    IndexHatDown = 20,
    ScrollFirst = 21,
    ScrollSecond = 22,
    ScrollDown = 23,
    ScrollUp = 24,
    ThumpHatPress = 25,
    ThumbHatUp = 26,
    ThumbHatRight = 27,
    ThumbHatDown = 28,
    ThumbHatLeft = 29,
    Pinky = 30,
    LowerTrigger = 31,
    GuardIn = 32,
}
impl ToButtonIndex for RightStickButtons {
    fn to_button_index(&self) -> u8 {
        *self as u8 - 1
    }
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum RightStickAxis {
    StickX = 1,
    StickY = 2,
    ThumbStickY = 3,
    ThumbStickX = 4,
    LowerTrigger = 5,
}
impl ToAxisIndex for RightStickAxis {
    fn to_axis_index(&self) -> u8 {
        *self as u8 - 1
    }
}
