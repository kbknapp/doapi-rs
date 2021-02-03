#[macro_use]
extern crate serde_derive;

mod domanager;
mod macros;

// pub mod request;
pub mod response;

pub use domanager::DoManager;
// pub use request::RequestBuilder;
// pub use request::DoRequest;

#[cfg(test)]
mod tests {}
