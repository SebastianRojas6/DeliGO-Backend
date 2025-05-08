use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "delivery_man")]

pub struct Model {
    #[sea_orm(primary_key)]
    pub id_delivery_man: i32,
    pub name: String,
    pub phone: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::order::Entity",
        from = "Column::IdDeliveryMan",
        to = "super::order::Column::IdDeliveryMan"
        
    )]
    Order,
}
impl ActiveModelBehavior for ActiveModel {}