// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(
    // Currently WIP and some code might not end up used
    dead_code,
    clippy::cognitive_complexity,
    clippy::module_inception,
)]

#[macro_use]
pub mod macros;
pub use self::macros::*;

pub mod args;
pub mod defaults;
pub mod errors;
pub mod lexer;
pub mod parser;
