
pub enum Reg8 { AL, BL, CL, DL, SIL, DIL, BPL, SPL, R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B }
pub enum Reg16 { AX, BX, CX, DX, SI, DI, BP, SP, R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W }
pub enum Reg32 { EAX, EBX, ECX, EDX, ESI, EDI, EBP, ESP, R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D }
pub enum Reg64 { RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP, R8, R9, R10, R11, R12, R13, R14, R15 }

pub trait Fragment {}
pub trait Label {}
pub trait Constant {}
pub trait Literal {}
pub trait Register {}
pub trait LocalVariable {}
pub trait GlobalVariable {}
pub trait StackVariable {}
pub trait Sized<const N: usize> {}

struct Nil {}
impl Fragment for Nil {}


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

/// Move with sign extend
pub fn movsx1(_src: impl Constant + Fragment + Register + Sized<1>, _dst: impl Constant + Fragment + Register + Sized<8>) -> impl Fragment {
   Nil {}
}
/// Move with sign extend
pub fn movsx2(_src: impl Constant + Fragment + Register + Sized<2>, _dst: impl Constant + Fragment + Register + Sized<8>) -> impl Fragment {
   Nil {}
}
/// Move with sign extend
pub fn movsx4(_src: impl Constant + Fragment + Register + Sized<4>, _dst: impl Constant + Fragment + Register + Sized<8>) -> impl Fragment {
   Nil {}
}

pub fn push1c(_src: impl Constant + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn push2c(_src: impl Constant + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn push4c(_src: impl Constant + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn push8c(_src: impl Constant + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn push1r(_src: impl Constant + Register + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn push2r(_src: impl Constant + Register + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn push4r(_src: impl Constant + Register + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn push8r(_src: impl Constant + Register + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn pushng<const N: usize>(_src: impl Constant + GlobalVariable + Sized<N>) -> impl Fragment {
   Nil {}
}
pub fn pushnl<const N: usize>(_src: impl Constant + LocalVariable + Sized<N>) -> impl Fragment {
   Nil {}
}
pub fn pushns<const N: usize>(_src: impl Constant + StackVariable + Sized<N>) -> impl Fragment {
   Nil {}
}

pub fn pop1r(_src: impl Constant + Register + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn pop2r(_src: impl Constant + Register + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn pop4r(_src: impl Constant + Register + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn pop8r(_src: impl Constant + Register + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn popng<const N: usize>(_src: impl Constant + GlobalVariable + Sized<N>) -> impl Fragment {
   Nil {}
}
pub fn popnl<const N: usize>(_src: impl Constant + LocalVariable + Sized<N>) -> impl Fragment {
   Nil {}
}
pub fn popns<const N: usize>(_src: impl Constant + StackVariable + Sized<N>) -> impl Fragment {
   Nil {}
}

pub fn mov1cr(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2cr(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4cr(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8cr(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn mov1cl(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2cl(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4cl(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8cl(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn mov1cg(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2cg(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4cg(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8cg(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn mov1cs(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2cs(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4cs(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8cs(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}


pub fn mov1lr(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2lr(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4lr(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8lr(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn mov1ll(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2ll(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4ll(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8ll(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn mov1lg(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2lg(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4lg(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8lg(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn mov1ls(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2ls(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4ls(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8ls(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}


pub fn mov1gr(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2gr(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4gr(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8gr(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn mov1gl(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2gl(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4gl(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8gl(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn mov1gg(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2gg(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4gg(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8gg(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn mov1gs(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2gs(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4gs(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8gs(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}

pub fn mov1sr(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2sr(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4sr(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8sr(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn mov1sl(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2sl(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4sl(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8sl(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn mov1sg(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2sg(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4sg(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8sg(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn mov1ss(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn mov2ss(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn mov4ss(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn mov8ss(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}

pub fn movngg<const N: usize>(_src: impl Constant + GlobalVariable + Fragment + Sized<N>, _dst: impl Constant + GlobalVariable + Fragment + Sized<N>) -> impl Fragment {
   Nil {}
}
pub fn movngl<const N: usize>(_src: impl Constant + GlobalVariable + Fragment + Sized<N>, _dst: impl Constant + LocalVariable + Fragment + Sized<N>) -> impl Fragment {
   Nil {}
}
pub fn movngs<const N: usize>(_src: impl Constant + GlobalVariable + Fragment + Sized<N>, _dst: impl Constant + StackVariable + Fragment + Sized<N>) -> impl Fragment {
   Nil {}
}
pub fn movnlg<const N: usize>(_src: impl Constant + LocalVariable + Fragment + Sized<N>, _dst: impl Constant + GlobalVariable + Fragment + Sized<N>) -> impl Fragment {
   Nil {}
}
pub fn movnll<const N: usize>(_src: impl Constant + LocalVariable + Fragment + Sized<N>, _dst: impl Constant + LocalVariable + Fragment + Sized<N>) -> impl Fragment {
   Nil {}
}
pub fn movnls<const N: usize>(_src: impl Constant + LocalVariable + Fragment + Sized<N>, _dst: impl Constant + StackVariable + Fragment + Sized<N>) -> impl Fragment {
   Nil {}
}
pub fn movnsg<const N: usize>(_src: impl Constant + StackVariable + Fragment + Sized<N>, _dst: impl Constant + GlobalVariable + Fragment + Sized<N>) -> impl Fragment {
   Nil {}
}
pub fn movnsl<const N: usize>(_src: impl Constant + StackVariable + Fragment + Sized<N>, _dst: impl Constant + LocalVariable + Fragment + Sized<N>) -> impl Fragment {
   Nil {}
}
pub fn movnss<const N: usize>(_src: impl Constant + StackVariable + Fragment + Sized<N>, _dst: impl Constant + StackVariable + Fragment + Sized<N>) -> impl Fragment {
   Nil {}
}

pub fn add1cc(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2cc(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4cc(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8cc(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1cr(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2cr(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4cr(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8cr(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1cg(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2cg(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4cg(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8cg(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1cl(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2cl(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4cl(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8cl(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1cs(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2cs(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4cs(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8cs(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}


pub fn add1rc(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2rc(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4rc(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8rc(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1rr(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2rr(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4rr(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8rr(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1rg(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2rg(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4rg(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8rg(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1rl(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2rl(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4rl(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8rl(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1rs(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2rs(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4rs(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8rs(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}


pub fn add1gc(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2gc(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4gc(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8gc(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1gr(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2gr(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4gr(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8gr(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1gg(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2gg(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4gg(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8gg(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1gl(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2gl(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4gl(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8gl(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1gs(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2gs(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4gs(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8gs(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}



pub fn add1lc(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2lc(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4lc(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8lc(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1lr(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2lr(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4lr(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8lr(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1lg(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2lg(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4lg(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8lg(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1ll(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2ll(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4ll(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8ll(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1ls(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2ls(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4ls(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8ls(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}


pub fn add1sc(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2sc(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4sc(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8sc(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1sr(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2sr(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4sr(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8sr(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1sg(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2sg(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4sg(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8sg(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1sl(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2sl(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4sl(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8sl(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn add1ss(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn add2ss(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn add4ss(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn add8ss(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}


