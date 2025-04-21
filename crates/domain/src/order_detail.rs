use sea_orm::entity::prelude::*;
use crate::order;
use crate::product;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "order_detail")]

pub struct Model {

    #[sea_orm(primary_key)]
    pub id_detail: i32,
    pub id_order: i32,
    pub id_product: i32,
    pub amount: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::order::Entity",
        from = "Column::IdOrder",
        to = "super::order::Column::IdOrder"
    )]
    Order,

    #[sea_orm(
        belongs_to = "super::product::Entity",
        from = "Column::IdProduct",
        to = "super::product::Column::IdProduct"
    )]
    Product,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl Related<product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}