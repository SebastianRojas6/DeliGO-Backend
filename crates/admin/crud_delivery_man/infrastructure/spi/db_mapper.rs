use crate::crud_delivery_man::domain::model::DeliveryManEntity;
use crate::crud_delivery_man::infrastructure::spi::entity::sea_orm_active_enums::map_roltype_from_entity;
use crate::crud_delivery_man::infrastructure::spi::entity::user::Model;

pub struct DBMapper;

impl DBMapper {
    pub fn to_entity(model: Model) -> DeliveryManEntity {
        DeliveryManEntity {
            id: model.id_user,
            name: model.name.unwrap_or_default(),
            phone: model.phone.unwrap_or_default(),
            password: model.password.unwrap_or_default(),
            email: String::new(),
            address: model.address.unwrap_or_default(),
            role: map_roltype_from_entity(model.rol),
            latitude: model.latitud.unwrap_or_default().parse().unwrap_or(0.0),
            longitude: model.longitud.unwrap_or_default().parse().unwrap_or(0.0),
        }
        
    }
}