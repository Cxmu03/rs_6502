use crate::registers::Flags;
use std::mem::transmute;

pub(crate) trait FromTwosComplementBits {
    type TwosComplementType;

    fn from_twos_complement_bits(t: Self::TwosComplementType) -> Self;
}

impl FromTwosComplementBits for i8 {
    type TwosComplementType = u8;

    fn from_twos_complement_bits(val: u8) -> i8 {
        unsafe {
            return transmute(val);
        }
    }
}

pub(crate) fn set_bit(value: u8, index: u8, value_at_index: bool) -> u8 {
    let mask = (value_at_index as u8) << index;

    if (value & (1 << index)) != mask {
        return toggle_bit(value, index);
    }

    value
}

pub(crate) fn toggle_bit(value: u8, index: u8) -> u8 {
    value ^ (1 << index)
}

pub(crate) fn get_bit(value: u8, index: u8) -> bool {
    value & (1 << index) == (1 << index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_bit() {
        let value = 0b00010000;

        assert_eq!(set_bit(value, 4, true), value);

        assert_eq!(set_bit(value, 4, false), 0u8);

        assert_eq!(set_bit(value, 4, false), 0u8);

        assert_eq!(set_bit(value, 4, true), value);
    }
}
