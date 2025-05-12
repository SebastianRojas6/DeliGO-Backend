#[derive(Debug, Clone)]
pub struct CreateUserDTOs {
    pub name: String,
    pub phone: String,
    pub address: String,
}

#[derive(Debug, Clone)]
pub struct UpdateUserDTOs {
    pub id_user: i32,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}