use crate::response::page::Pages;

#[derive(Deserialize)]
pub struct Links {
    pub pages: Option<Pages>,
}
impl Default for Links {
    fn default() -> Links {
        Links { pages: None }
    }
}
