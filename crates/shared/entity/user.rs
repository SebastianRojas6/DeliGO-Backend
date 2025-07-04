//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.12

use super::sea_orm_active_enums::RolType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_user: i32,
    pub name: Option<String>,
    pub phone: String,
    pub address: String,
    pub rol: RolType,
    pub latitud: Option<String>,
    pub longitud: Option<String>,
    pub password: String,
    #[sea_orm(unique)]
    pub email: String,
    #[sea_orm(column_type = "Float", nullable)]
    pub rating: Option<f32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::order::Entity")]
    Order,
    #[sea_orm(has_many = "super::shopping_cart::Entity")]
    ShoppingCart,
}

impl Related<super::order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl Related<super::shopping_cart::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ShoppingCart.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
