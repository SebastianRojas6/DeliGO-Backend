use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserFactsPresenter {
    #[serde(rename = "identifier")]
    pub id_user: i32,
    pub name: String,
    pub phone: String,
    pub address: String,
}