/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

*/

#![feature(thread_id_value)]

pub mod s; pub use crate::s::*; //S-Expression Library
pub mod a; pub use crate::a::*; //S-Expression based AST helper functions
pub mod p; pub use crate::p::*; //Quickly parse input into AST
pub mod e; pub use crate::e::*; //Evaluate expressions
pub mod g; pub use crate::g::*; //Generate Code and Compile
