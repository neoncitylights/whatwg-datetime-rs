#![doc = include_str!("../README.md")]

mod components;
mod utils;

pub use crate::components::*;

pub type ParseStringFn<T> = dyn Fn(&str) -> Option<T>;
pub type ParseComponentFn<T> = dyn Fn(&str, &mut usize) -> Option<T>;
