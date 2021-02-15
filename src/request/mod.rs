pub use builder::{DnsRecord, RequestBuilder};

pub use dorequest::{BaseRequest, DoRequest};
pub use page::PagedRequest;

mod builder;
mod dorequest;
mod page;
