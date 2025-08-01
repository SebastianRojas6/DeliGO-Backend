//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.12

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_product: i32,
    pub name: Option<String>,
    pub price: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::order_details::Entity")]
    OrderDetails,
    #[sea_orm(has_many = "super::shopping_cart::Entity")]
    ShoppingCart,
}

impl Related<super::order_details::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrderDetails.def()
    }
}

impl Related<super::shopping_cart::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ShoppingCart.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
