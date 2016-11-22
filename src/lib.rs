#![crate_type= "lib"]
#![feature(custom_derive,
           custom_attribute,
           plugin)]
#![plugin(serde_macros)]
#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", allow(explicit_iter_loop))]
#![cfg_attr(feature = "lints", allow(should_implement_trait))]
#![cfg_attr(feature = "lints", deny(warnings))]
#![deny(trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unused_import_braces)]

extern crate serde;
extern crate serde_json;
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
mod tests {}
