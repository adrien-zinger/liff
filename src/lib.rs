#![feature(test)]
mod apply;
mod diff;
mod diffio;

pub use apply::apply;
pub use diff::diff;

pub use diffio::{debug, debug_u8_to_char, read, write, write_char};

#[cfg(test)]
mod tests;
