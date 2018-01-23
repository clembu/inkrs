#![feature(conservative_impl_trait)]
extern crate itertools;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[cfg(test)]
extern crate serde_test;

pub mod error;
#[macro_use]
mod json;
mod tree;
