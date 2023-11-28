use std::mem::transmute;

pub(crate) trait ToI8<T> {
    fn to_i8(&self) -> T;
}

impl ToI8<i8> for u8 {
    fn to_i8(&self) -> i8 {
        unsafe {
            return transmute(*self);
        }
    }
}
