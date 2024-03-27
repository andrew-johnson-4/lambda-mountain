
pub enum Reg8 { AL, BL, CL, DL, SIL, DIL, BPL, SPL, R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B }
pub enum Reg16 { AX, BX, CX, DX, SI, DI, BP, SP, R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W }
pub enum Reg32 { EAX, EBX, ECX, EDX, ESI, EDI, EBP, ESP, R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D }
pub enum Reg64 { RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP, R8, R9, R10, R11, R12, R13, R14, R15 }

pub enum FragmentType {
   And(Vec<FragmentType>),
   Constructor(String,Vec<FragmentType>),
   Arrow(Box<FragmentType>,Box<FragmentType>),
}

//Rust can't infer the return type of panic! in this case so unreachable code is necessary to compile
#[allow(unreachable_code)]
pub trait Fragment {
   fn fragment_type(&self) -> FragmentType { panic!("Fragment is untyped") }
   fn as_literal(&self) -> impl Fragment + Constant + Literal { panic!("Fragment is not a constant literal"); Nil {} }
   fn as_register(&self) -> impl Fragment + Constant + Register { panic!("Fragment is not a constant register"); Nil {} }
   fn as_global_variable(&self) -> impl Fragment + Constant + GlobalVariable { panic!("Fragment is not a constant global variable"); Nil {} }
   fn as_local_variable(&self) -> impl Fragment + Constant + LocalVariable { panic!("Fragment is not a constant local variable"); Nil {} }
   fn as_stack_variable(&self) -> impl Fragment + Constant + StackVariable { panic!("Fragment is not a constant stack variable"); Nil {} }
}

pub trait Program { fn program(&self) -> String; }
pub trait Text { fn text(&self) -> String; }
pub trait Data { fn data(&self) -> String; }
pub trait Frame { fn frame(&self) -> String; }
pub trait Unframe { fn unframe(&self) -> String; }
pub trait Type { fn fragment_type(&self) -> FragmentType; }
pub trait Label { fn label_id(&self) -> String; }
pub trait FrameSize { fn frame_size(&self) -> usize; }
pub trait Constant {}
pub trait Literal { fn literal_value(&self) -> String; }
pub trait Register { fn register_name(&self) -> String; }
pub trait LocalVariable { fn offset_from_base_pointer(&self) -> i64; }
pub trait GlobalVariable { fn global_variable_identifier(&self) -> String; }
pub trait StackVariable { fn offset_from_stack_pointer(&self) -> i64; }
pub trait ConditionalJump { fn jump_if_true(&self, label_id: String) -> String; fn jump_if_false(&self, label_id: &String) -> String; }
pub trait Sized<const N: usize> {}

pub struct Nil {}
impl Fragment for Nil {}
impl Program for Nil { fn program(&self) -> String { panic!("Nil::Program") } }
impl Literal for Nil { fn literal_value(&self) -> String { panic!("Nil::Literal") } }
impl Label for Nil { fn label_id(&self) -> String { panic!("Nil::Label") } }
impl Register for Nil { fn register_name(&self) -> String { panic!("Nil::Register") } }
impl GlobalVariable for Nil { fn global_variable_identifier(&self) -> String { panic!("Nil::GlobalVariable") } }
impl LocalVariable for Nil { fn offset_from_base_pointer(&self) -> i64 { panic!("Nil::LocalVariable") } }
impl StackVariable for Nil { fn offset_from_stack_pointer(&self) -> i64 { panic!("Nil::StackVariable") } }
impl Constant for Nil { }

pub struct ProgramFragment { pub program: String }
impl Fragment for ProgramFragment {}
impl Program for ProgramFragment { fn program(&self) -> String { return self.program.clone(); } }
