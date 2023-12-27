/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

G: A Basic Codegen

*/

use crate::*;
use punc::*;

pub fn compile(cfg: &str, s: &S) {
  punc!(
      (.global _start)
      (.text)
      (label _start
         (mov @60 %rax)
         (xor %rdi %rdi)
         (syscall)
      )
   ).compile(cfg);
}
