#![feature(custom_derive, custom_attribute, append, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate hyper;
extern crate regex;

mod domanager;
#[macro_use]
mod macros;

pub mod request;
pub mod response;

pub use domanager::DoManager;
pub use request::RequestBuilder;
pub use request::DoRequest;

#[cfg(test)]
mod tests{
}
