
#[derive(serde::Serialize)]
pub struct Response{
    pub total: i64,
    pub items: Option<Vec<String>>,
}