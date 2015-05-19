#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate hyper;

mod domanager;
pub mod request;
pub mod response;

pub use domanager::DoManager;

#[cfg(test)]
mod tests{
}
