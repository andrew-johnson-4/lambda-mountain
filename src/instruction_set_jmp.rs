
use crate::*;

/// Jump if overflow
pub fn jo(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjo {}\n", dst.label_id()) }
}

/// Jump if not overflow
pub fn jno(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjno {}\n", dst.label_id()) }
}

/// Jump if sign
pub fn js(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjs {}\n", dst.label_id()) }
}

/// Jump if not sign
pub fn jns(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjns {}\n", dst.label_id()) }
}

/// Jump if equal
pub fn je(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tje {}\n", dst.label_id()) }
}

/// Jump if zero
pub fn jz(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjz {}\n", dst.label_id()) }
}

/// Jump if not equal
pub fn jne(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjne {}\n", dst.label_id()) }
}

/// Jump if not zero
pub fn jnz(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjnz {}\n", dst.label_id()) }
}

/// Jump if below (unsigned)
pub fn jb(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjb {}\n", dst.label_id()) }
}

/// Jump if not above of equal (unsigned)
pub fn jnae(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjnae {}\n", dst.label_id()) }
}

/// Jump if carry
pub fn jc(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjc {}\n", dst.label_id()) }
}

/// Jump if not below (unsigned)
pub fn jnb(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjnb {}\n", dst.label_id()) }
}

/// Jump if above or equal (unsigned)
pub fn jae(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjae {}\n", dst.label_id()) }
}

/// Jump if not carry
pub fn jnc(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjnc {}\n", dst.label_id()) }
}

/// Jump if below or equal (unsigned)
pub fn jbe(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjbe {}\n", dst.label_id()) }
}

/// Jump if not above (unsigned)
pub fn jna(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjna {}\n", dst.label_id()) }
}

/// Jump if above (unsigned)
pub fn ja(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tja {}\n", dst.label_id()) }
}

/// Jump if not below or equal (unsigned)
pub fn jnbe(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjnbe {}\n", dst.label_id()) }
}

/// Jump if less (signed)
pub fn jl(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjl {}\n", dst.label_id()) }
}

/// Jump if not greater or equal (signed)
pub fn jnge(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjnge {}\n", dst.label_id()) }
}

/// Jump if greater or equal (signed)
pub fn jge(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjge {}\n", dst.label_id()) }
}

/// Jump if not less (signed)
pub fn jnl(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjnl {}\n", dst.label_id()) }
}

/// Jump if less or equal (signed)
pub fn jle(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjle {}\n", dst.label_id()) }
}

/// Jump if not greater (signed)
pub fn jng(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjng {}\n", dst.label_id()) }
}

/// Jump if greater (signed)
pub fn jg(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjg {}\n", dst.label_id()) }
}

/// Jump if not less or equal (signed)
pub fn jnle(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjnle {}\n", dst.label_id()) }
}

/// Jump if parity
pub fn jp(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjp {}\n", dst.label_id()) }
}

/// Jump if parity even
pub fn jpe(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjpe {}\n", dst.label_id()) }
}

/// Jump if not parity
pub fn jnp(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjnp {}\n", dst.label_id()) }
}

/// Jump if parity odd
pub fn jpo(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjpo {}\n", dst.label_id()) }
}

/// Jump if %CX register is zero
pub fn jcxz(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjcxz {}\n", dst.label_id()) }
}

/// Jump if %ECX register is zero
pub fn jecxz(dst: impl Constant + Fragment + Label) -> impl Fragment + Program {
   ProgramFragment { program: format!("\tjecxz {}\n", dst.label_id()) }
}

