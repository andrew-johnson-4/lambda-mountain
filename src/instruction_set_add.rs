
use crate::*;

pub fn add1cc(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2cc(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4cc(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8cc(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1cr(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2cr(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4cr(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8cr(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1cg(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2cg(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4cg(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8cg(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1cl(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2cl(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4cl(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8cl(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1cs(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2cs(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4cs(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8cs(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}


pub fn add1rc(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2rc(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4rc(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8rc(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1rr(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2rr(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4rr(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8rr(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1rg(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2rg(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4rg(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8rg(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1rl(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2rl(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4rl(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8rl(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1rs(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2rs(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4rs(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8rs(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}


pub fn add1gc(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2gc(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4gc(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8gc(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1gr(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2gr(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4gr(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8gr(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1gg(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2gg(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4gg(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8gg(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1gl(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2gl(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4gl(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8gl(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1gs(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2gs(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4gs(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8gs(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}



pub fn add1lc(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2lc(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4lc(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8lc(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1lr(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2lr(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4lr(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8lr(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1lg(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2lg(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4lg(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8lg(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1ll(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2ll(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4ll(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8ll(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1ls(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2ls(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4ls(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8ls(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}


pub fn add1sc(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2sc(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4sc(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8sc(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1sr(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2sr(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4sr(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8sr(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1sg(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2sg(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4sg(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8sg(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1sl(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2sl(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4sl(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8sl(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn add1ss(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn add2ss(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn add4ss(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn add8ss(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}


