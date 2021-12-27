#![feature(no_core)]
#![feature(lang_items)]
#![feature(negative_impls)]
#![feature(intrinsics)]
#![feature(prelude_import)]
#![feature(rustc_attrs)]
#![feature(decl_macro)]
#![feature(auto_traits)]
#![feature(exhaustive_patterns)]

#![no_core]

#[prelude_import]
#[allow(unused)]
use prelude::rust_2021::*;

pub mod alloc;
pub mod clone;
pub mod cmp;
pub mod convert;
pub mod intrinsics;
pub mod marker;
pub mod mem;
pub mod num;
pub mod ops;
pub mod option;
pub mod prelude;
pub mod ptr;
pub mod result;
pub mod slice;
pub mod str;

#[macro_use]
mod macros;

pub mod arch {
    #[rustc_builtin_macro]
    pub macro asm("assembly template", $(operands,)* $(options($(option),*))?) {
        /* compiler built-in */
    }
}