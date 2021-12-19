#![feature(test)]
mod apply;
mod diff;
mod diffio;

pub use diff::diff;
pub use apply::apply;

pub use diffio::{
    debug,
    debug_u8_to_char,
    write,
    write_char,
    read,
};

#[cfg(test)]
mod tests;
