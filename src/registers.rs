#[repr(u8)]
pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}

impl Register {
    pub const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Registers {
    x: [u32; 32],
    pub ip: u32,
}

impl Registers {
    pub fn new() -> Self {
        Self { x: [0; 32], ip: 0 }
    }

    #[inline(always)]
    pub fn read(&self, r: Register) -> u32 {
        self.x[r.index()]
    }

    #[inline(always)]
    pub fn write(&mut self, r: Register, val: u32) {
        // we don't alter the x0 register.
        if let Register::X0 = r {
            return;
        }

        self.x[r.index()] = val;
    }
}
