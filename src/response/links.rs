use response::page::Pages;

#[derive(Deserialize)]
pub struct Links {
    pub pages: Option<Pages>,
}
