
use crate::*;

pub fn mov1cr(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2cr(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4cr(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8cr(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov1cl(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2cl(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4cl(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8cl(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov1cg(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2cg(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4cg(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8cg(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov1cs(_src: impl Constant + Literal + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2cs(_src: impl Constant + Literal + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4cs(_src: impl Constant + Literal + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8cs(_src: impl Constant + Literal + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}


pub fn mov1lr(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2lr(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4lr(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8lr(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov1ll(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2ll(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4ll(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8ll(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov1lg(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2lg(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4lg(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8lg(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov1ls(_src: impl Constant + LocalVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2ls(_src: impl Constant + LocalVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4ls(_src: impl Constant + LocalVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8ls(_src: impl Constant + LocalVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}


pub fn mov1gr(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2gr(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4gr(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8gr(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov1gl(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2gl(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4gl(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8gl(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov1gg(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2gg(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4gg(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8gg(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov1gs(_src: impl Constant + GlobalVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2gs(_src: impl Constant + GlobalVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4gs(_src: impl Constant + GlobalVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8gs(_src: impl Constant + GlobalVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}

pub fn mov1sr(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + Register + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2sr(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + Register + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4sr(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + Register + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8sr(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + Register + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov1sl(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + LocalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2sl(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + LocalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4sl(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + LocalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8sl(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + LocalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov1sg(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + GlobalVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2sg(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + GlobalVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4sg(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + GlobalVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8sg(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + GlobalVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov1ss(_src: impl Constant + StackVariable + Fragment + Sized<1>, _dst: impl Constant + StackVariable + Fragment + Sized<1>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov2ss(_src: impl Constant + StackVariable + Fragment + Sized<2>, _dst: impl Constant + StackVariable + Fragment + Sized<2>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov4ss(_src: impl Constant + StackVariable + Fragment + Sized<4>, _dst: impl Constant + StackVariable + Fragment + Sized<4>) -> impl Fragment + Program {
   Nil {}
}
pub fn mov8ss(_src: impl Constant + StackVariable + Fragment + Sized<8>, _dst: impl Constant + StackVariable + Fragment + Sized<8>) -> impl Fragment + Program {
   Nil {}
}

pub fn movngg<const N: usize>(_src: impl Constant + GlobalVariable + Fragment + Sized<N>, _dst: impl Constant + GlobalVariable + Fragment + Sized<N>) -> impl Fragment + Program {
   Nil {}
}
pub fn movngl<const N: usize>(_src: impl Constant + GlobalVariable + Fragment + Sized<N>, _dst: impl Constant + LocalVariable + Fragment + Sized<N>) -> impl Fragment + Program {
   Nil {}
}
pub fn movngs<const N: usize>(_src: impl Constant + GlobalVariable + Fragment + Sized<N>, _dst: impl Constant + StackVariable + Fragment + Sized<N>) -> impl Fragment + Program {
   Nil {}
}
pub fn movnlg<const N: usize>(_src: impl Constant + LocalVariable + Fragment + Sized<N>, _dst: impl Constant + GlobalVariable + Fragment + Sized<N>) -> impl Fragment + Program {
   Nil {}
}
pub fn movnll<const N: usize>(_src: impl Constant + LocalVariable + Fragment + Sized<N>, _dst: impl Constant + LocalVariable + Fragment + Sized<N>) -> impl Fragment + Program {
   Nil {}
}
pub fn movnls<const N: usize>(_src: impl Constant + LocalVariable + Fragment + Sized<N>, _dst: impl Constant + StackVariable + Fragment + Sized<N>) -> impl Fragment + Program {
   Nil {}
}
pub fn movnsg<const N: usize>(_src: impl Constant + StackVariable + Fragment + Sized<N>, _dst: impl Constant + GlobalVariable + Fragment + Sized<N>) -> impl Fragment + Program {
   Nil {}
}
pub fn movnsl<const N: usize>(_src: impl Constant + StackVariable + Fragment + Sized<N>, _dst: impl Constant + LocalVariable + Fragment + Sized<N>) -> impl Fragment + Program {
   Nil {}
}
pub fn movnss<const N: usize>(_src: impl Constant + StackVariable + Fragment + Sized<N>, _dst: impl Constant + StackVariable + Fragment + Sized<N>) -> impl Fragment + Program {
   Nil {}
}

