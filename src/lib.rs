#[macro_use]
extern crate serde;

mod domanager;

/*#[macro_use]
mod macros;*/

pub mod request;
pub mod response;

pub use domanager::DoManager;
//pub use request::RequestBuilder;
pub use request::DoRequest;

#[cfg(test)]
pub(crate) mod tests {
    pub const BEARER_HEADER: &str =
        "Bearer 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    pub const TEST_TOKEN: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
}
