
use crate::*;

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

