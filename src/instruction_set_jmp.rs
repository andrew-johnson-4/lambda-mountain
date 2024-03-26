
use crate::*;

/// Jump if overflow
pub fn jo(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if not overflow
pub fn jno(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if sign
pub fn js(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if not sign
pub fn jns(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if equal
pub fn je(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if zero
pub fn jz(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if not equal
pub fn jne(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if not zero
pub fn jnz(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if below (unsigned)
pub fn jb(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if not above of equal (unsigned)
pub fn jnae(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if carry
pub fn jc(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if not below (unsigned)
pub fn jnb(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if above or equal (unsigned)
pub fn jae(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if not carry
pub fn jnc(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if below or equal (unsigned)
pub fn jbe(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if not above (unsigned)
pub fn jna(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if above (unsigned)
pub fn ja(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if not below or equal (unsigned)
pub fn jnbe(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if less (signed)
pub fn jl(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if not greater or equal (signed)
pub fn jnge(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if greater or equal (signed)
pub fn jge(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if not less (signed)
pub fn jnl(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if less or equal (signed)
pub fn jle(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if not greater (signed)
pub fn jng(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if greater (signed)
pub fn jg(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if not less or equal (signed)
pub fn jnle(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if parity
pub fn jp(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if parity even
pub fn jpe(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if not parity
pub fn jnp(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if parity odd
pub fn jpo(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if %CX register is zero
pub fn jcxz(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

/// Jump if %ECX register is zero
pub fn jecxz(_dst: impl Constant + Fragment + Label) -> impl Fragment {
   Nil {}
}

