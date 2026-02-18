#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::unnecessary_cast, clippy::useless_transmute)]
#![allow(unnecessary_transmutes)]

mod bindings;
pub use bindings::*;

pub mod loader;
