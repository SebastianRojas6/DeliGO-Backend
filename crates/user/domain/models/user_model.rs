use super::enums::RoleType;
#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub address: String,
    pub rol: RoleType,
    pub longitude: String,
    pub latitude: String,
}
