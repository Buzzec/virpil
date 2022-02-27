use strum::{EnumCount, EnumIter};

use crate::virpil_device::{ToAxisIndex, ToButtonIndex};
use crate::{BoardType, ToBoardAndLedNumber, VirpilDeviceDescription};

pub struct Throttle;
impl VirpilDeviceDescription for Throttle {
    type Led = ThrottleLed;
    type Buttons = ThrottleButtons;
    type Axis = ThrottleAxis;
    const PID: u16 = 0x0194;
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum ThrottleLed {
    B1 = 1,
    B2 = 2,
    B3 = 3,
    B4 = 4,
    B5 = 5,
    B6 = 6,
}
impl ToBoardAndLedNumber for ThrottleLed {
    fn to_board_and_led_number(&self) -> (BoardType, u8) {
        (BoardType::OnBoard, *self as u8)
    }
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum ThrottleButtons {
    PinkyDialPress = 1,
    PinkyDialReverse = 2,
    PinkyDialForward = 3,
    PinkyButton = 4,
    RingHatPress = 5,
    RingHatDown = 6,
    RingHatUp = 7,
    IndexHatPress = 8,
    IndexHatDown = 9,
    IndexHatRight = 10,
    IndexHatUp = 11,
    IndexHatLeft = 12,
    StickPress = 13,
    ThumbWheelForward = 14,
    ThumbWheelBackward = 15,
    WheelHatPress = 16,
    WheelHatForward = 17,
    WheelHatDown = 18,
    WheelHatBackward = 19,
    WheelHatUp = 20,
    ThumbFrontButton = 21,
    UpperHatPress = 22,
    UpperHatDown = 23,
    UpperHatBackward = 24,
    UpperHatUp = 25,
    UpperHatForward = 26,
    LowerHatPress = 27,
    LowerHatDown = 28,
    LowerHatBackward = 29,
    LowerHatUp = 30,
    LowerHatForward = 31,
    ThumbBackButton = 32,
    ThumbLowerButton = 33,
    T1Up = 34,
    T2Up = 35,
    T3Up = 36,
    T4Up = 37,
    B1 = 38,
    B2 = 39,
    B3 = 40,
    B4 = 41,
    B5 = 42,
    B6 = 43,
    T5Up = 44,
    T5Down = 45,
    T6Up = 46,
    T6Down = 47,
    T7Up = 48,
    T7Down = 49,
    E1Press = 50,
    E1CounterClockwise = 51,
    E1Clockwise = 52,
    E2Press = 53,
    E2CounterClockwise = 54,
    E2Clockwise = 55,
    Mode1 = 56,
    Mode2 = 57,
    Mode3 = 58,
    Mode4 = 59,
    Mode5 = 60,
    T1Down = 61,
    T2Down = 62,
    T3Down = 63,
    T4Down = 64,
    LeftThrottleZero = 65,
    LeftThrottleAfter = 66,
    RightThrottleZero = 67,
    RightThrottleAfter = 68,
    FlapsDown = 69,
    FlapsMiddle = 70,
    FlapsUp = 71,
    SliderDown = 72,
    SliderMiddle = 73,
    SliderUp = 74,
    ThrottlesLinked = 75,
    ThrottlesUnlinked = 76,
    LeftThrottleNonZero = 77,
    RightThrottleNonZero = 78,
}
impl ToButtonIndex for ThrottleButtons {
    fn to_button_index(&self) -> u8 {
        *self as u8 - 1
    }
}

#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum ThrottleAxis {
    LeftThrottle = 1,
    RightThrottle = 2,
    Flaps = 3,
    StickX = 4,
    StickY = 5,
    Slider = 6,
}
impl ToAxisIndex for ThrottleAxis {
    fn to_axis_index(&self) -> u8 {
        *self as u8 - 1
    }
}
