#![feature(custom_derive, custom_attribute, collections, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate hyper;

mod domanager;
pub mod request;
pub mod response;

pub use domanager::DoManager;
pub use request::Request;

#[cfg(test)]
mod tests{
}
