use sea_orm::entity::prelude::*;

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
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}