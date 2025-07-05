#[derive(Debug, Clone)]
pub struct UserCredential {
    pub id: Option<i32>, // Será None en el registro
    pub name: String,
    pub phone: String,
    pub password: String,
    pub email: String,
    pub address: String,
}
