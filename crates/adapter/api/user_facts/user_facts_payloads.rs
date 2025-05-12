use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserFactPayload {
    pub name: String,
    pub phone: String,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserFactPayload {
    pub id_user: i32,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}