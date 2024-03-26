
use crate::*;

pub fn or1cc(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2cc(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4cc(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8cc(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1cr(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2cr(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4cr(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8cr(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1cg(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2cg(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4cg(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8cg(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1cl(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2cl(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4cl(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8cl(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1cs(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2cs(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4cs(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8cs(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}


pub fn or1rc(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2rc(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4rc(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8rc(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1rr(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2rr(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4rr(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8rr(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1rg(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2rg(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4rg(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8rg(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1rl(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2rl(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4rl(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8rl(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1rs(_src: impl Constant + Register + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2rs(_src: impl Constant + Register + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4rs(_src: impl Constant + Register + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8rs(_src: impl Constant + Register + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}


pub fn or1gc(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2gc(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4gc(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8gc(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1gr(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2gr(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4gr(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8gr(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1gg(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2gg(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4gg(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8gg(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1gl(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2gl(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4gl(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8gl(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1gs(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2gs(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4gs(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8gs(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}



pub fn or1lc(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2lc(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4lc(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8lc(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1lr(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2lr(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4lr(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8lr(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1lg(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2lg(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4lg(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8lg(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1ll(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2ll(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4ll(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8ll(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1ls(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2ls(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4ls(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8ls(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}


pub fn or1sc(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + Literal + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2sc(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + Literal + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4sc(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + Literal + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8sc(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + Literal + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1sr(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2sr(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4sr(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8sr(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1sg(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2sg(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4sg(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8sg(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1sl(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2sl(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4sl(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8sl(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}
pub fn or1ss(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment {
   Nil {}
}
pub fn or2ss(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment {
   Nil {}
}
pub fn or4ss(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment {
   Nil {}
}
pub fn or8ss(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment {
   Nil {}
}


