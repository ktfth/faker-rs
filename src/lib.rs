#![feature(phase)]
extern crate regex;
#[phase(plugin)] 
extern crate regex_macros;

pub mod random;
pub mod name;
