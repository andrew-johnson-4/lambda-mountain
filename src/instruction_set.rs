
pub enum Reg8 { AL, BL, CL, DL, SIL, DIL, BPL, SPL, R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B }
pub enum Reg16 { AX, BX, CX, DX, SI, DI, BP, SP, R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W }
pub enum Reg32 { EAX, EBX, ECX, EDX, ESI, EDI, EBP, ESP, R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D }
pub enum Reg64 { RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP, R8, R9, R10, R11, R12, R13, R14, R15 }

pub trait Fragment {}
pub trait Constant {}
pub trait Literal {}
pub trait Register {}
pub trait LocalVariable {}
pub trait GlobalVariable {}
pub trait Sized<const N: usize> {}

struct Nil {}
impl Fragment for Nil {}

pub fn push1(_src: impl Constant + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn push2(_src: impl Constant + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn push4(_src: impl Constant + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn push8(_src: impl Constant + Fragment + Sized<8>) -> impl Fragment {
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


