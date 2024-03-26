
use crate::*;

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

