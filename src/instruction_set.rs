
pub enum Reg8 { AL, BL, CL, DL, SIL, DIL, BPL, SPL, R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B }
pub enum Reg16 { AX, BX, CX, DX, SI, DI, BP, SP, R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W }
pub enum Reg32 { EAX, EBX, ECX, EDX, ESI, EDI, EBP, ESP, R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D }
pub enum Reg64 { RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP, R8, R9, R10, R11, R12, R13, R14, R15 }

pub trait Fragment {}
pub trait Label { fn label_id() -> String; }
pub trait Constant {}
pub trait Literal { fn literal_value() -> String; }
pub trait Register { fn register_name() -> String; }
pub trait LocalVariable { fn offset_from_base_pointer() -> usize; }
pub trait GlobalVariable { fn global_variable_identifier() -> String; }
pub trait StackVariable { fn offset_from_stack_pointer() -> usize; }
pub trait Sized<const N: usize> {}

pub struct Nil {}
impl Fragment for Nil {}

