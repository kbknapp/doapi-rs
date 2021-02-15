use url::Url;

use crate::response::RawPagedResponse;

pub trait PagedRequest {
    type Item;
    fn retrieve_single_page(&self, url: Url) -> Result<RawPagedResponse<Self::Item>, String>;
}
