use super::super::domain::models::enums::RoleType;
use shared::entity::sea_orm_active_enums::RolType;

// Por ahora solo hice para el Rol, si necesitan para otros enums ya lo unifican bien la organización de carpetas

pub fn rol_domain_to_db(role: RoleType) -> RolType {
    match role {
        RoleType::Admin => RolType::Administrator,
        RoleType::Delivery => RolType::Delivery,
        RoleType::Customer => RolType::Customer,
    }
}

pub fn rol_db_to_domain(rol: RolType) -> RoleType {
    match rol {
        RolType::Administrator => RoleType::Admin,
        RolType::Delivery => RoleType::Delivery,
        RolType::Customer => RoleType::Customer,
    }
}
