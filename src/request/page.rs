use crate::response::RawPagedResponse;

pub trait PagedRequest {
    type Item;
    fn retrieve_single_page(&self, url: String) -> Result<RawPagedResponse<Self::Item>, String>;
}
