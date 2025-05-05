#[derive()]
pub struct UserEntity {
    pub id_user: i32,
    pub name: String,
    pub phone: String,
    pub address: String,
}

impl UserEntity {
    pub fn new(
        id_user: i32,
        name: String,
        phone: String,
        address: String,
    ) -> Self {
        Self {
            id_user,
            name,
            phone,
            address,
        }
    }
    pub fn get_id(&self) -> i32 {
        self.id_user
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_phone(&self) -> &str {
        &self.phone
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }
}