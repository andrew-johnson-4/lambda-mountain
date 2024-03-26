
use crate::*;

pub fn sub1cc(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2cc(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4cc(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8cc(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1cr(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2cr(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4cr(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8cr(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1cg(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2cg(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4cg(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8cg(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1cl(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2cl(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4cl(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8cl(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1cs(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2cs(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4cs(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8cs(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}


pub fn sub1rc(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2rc(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4rc(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8rc(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1rr(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2rr(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4rr(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8rr(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1rg(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2rg(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4rg(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8rg(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1rl(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2rl(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4rl(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8rl(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1rs(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2rs(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4rs(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8rs(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}


pub fn sub1gc(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2gc(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4gc(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8gc(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1gr(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2gr(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4gr(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8gr(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1gg(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2gg(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4gg(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8gg(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1gl(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2gl(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4gl(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8gl(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1gs(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2gs(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4gs(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8gs(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}



pub fn sub1lc(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2lc(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4lc(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8lc(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1lr(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2lr(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4lr(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8lr(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1lg(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2lg(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4lg(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8lg(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1ll(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2ll(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4ll(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8ll(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1ls(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2ls(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4ls(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8ls(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}


pub fn sub1sc(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2sc(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4sc(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8sc(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1sr(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2sr(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4sr(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8sr(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1sg(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2sg(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4sg(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8sg(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1sl(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2sl(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4sl(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8sl(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn sub1ss(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn sub2ss(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn sub4ss(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn sub8ss(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}


