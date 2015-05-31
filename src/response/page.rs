use response::{Meta, Links};

#[derive(Deserialize, Debug, Clone)]
pub struct Pages {
    pub first: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub last: Option<String>
}

// #[derive(Deserialize)]
// pub struct PagedResponse<T>(pub Vec<T>);

// impl<T> PagedResponse<T> {
//     pub fn iter(&self) -> Iter<T> {
//         self.0.iter()
//     }
// }

#[derive(Deserialize)]
pub struct RawPagedResponse<T> {
    pub collection: Vec<T>,
    pub links: Links,
    pub meta: Meta
}

pub trait NewIter {
    type Item;
    fn new() -> Vec<Self::Item> {
        vec![]
    }
}

impl<R> NewIter for R
              where R: Iterator {
    type Item = <Self as Iterator>::Item;
}