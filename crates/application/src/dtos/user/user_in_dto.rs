#[derive(Clone, Debug)]
pub struct UserCreateInDTO {
    pub name: String,
    pub phone: String,
    pub address: String,
}

#[derive(Clone, Debug)]
pub struct UserUpdateInDTO {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}