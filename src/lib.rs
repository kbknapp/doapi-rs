#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate hyper;

mod domanager;
mod request;
mod response;

pub use domanager::DoManager;

#[cfg(test)]
mod tests{
}
