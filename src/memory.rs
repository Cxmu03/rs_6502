pub trait ByteData<const N: usize>: Copy {
    fn from_bytes(bytes: [u8; N]) -> Self;
    fn to_bytes(self) -> [u8; N];
}

impl ByteData<1> for u8 {
    fn from_bytes(bytes: [u8; 1]) -> Self {
        bytes[0]
    }

    fn to_bytes(self) -> [u8; 1] {
        [self]
    }
}

impl ByteData<2> for u16 {
    fn from_bytes(bytes: [u8; 2]) -> Self {
        ((bytes[1] as u16) << 8) | bytes[0] as u16
    }

    fn to_bytes(self) -> [u8; 2] {
        [(self & 0xFF) as u8, (self >> 8) as u8]
    }
}

pub struct DefaultMemory {
    data: [u8; 0x10000] // 64kb of ram
}

impl DefaultMemory {
    pub fn new() -> DefaultMemory {
        DefaultMemory {
            data: [0; 0x10000]
        }
    }

    pub fn is_valid_address(address: u16) -> bool {
        return address < u16::MAX; 
    } 

    pub fn read<T: ByteData<N>, const N: usize>(&self, address: u16) -> T {
        let end_address = address as usize + N;
        if !DefaultMemory::is_valid_address(address) {
            panic!("Invalid address");
        }

        T::from_bytes(self.data[address as usize..end_address].try_into().expect("must be of size N"))
    }
    
    pub fn write<T: ByteData<N>, const N: usize>(&mut self, address: u16, t: T) {
        let end_address = address as usize + N;
        if !DefaultMemory::is_valid_address(address) {
            panic!("Invalid address");
        }

        self.data[(address as usize)..end_address].copy_from_slice(&t.to_bytes());
    }
}

