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
