//! Priority mask register

/// All exceptions with configurable priority are ...
#[allow(clippy::missing_inline_in_public_items)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Primask {
    /// Active
    Active,
    /// Inactive
    Inactive,
}

impl Primask {
    /// All exceptions with configurable priority are active
    #[inline]
    pub fn is_active(self) -> bool {
        self == Primask::Active
    }

    /// All exceptions with configurable priority are inactive
    #[inline]
    pub fn is_inactive(self) -> bool {
        self == Primask::Inactive
    }
}

/// Reads the CPU register
#[inline]
pub fn read() -> Primask {
    fn read_raw() -> u32 {
        call_asm!(__primask_r() -> u32)
    }

    let r = read_raw();
    if r & (1 << 0) == (1 << 0) {
        Primask::Inactive
    } else {
        Primask::Active
    }
}
