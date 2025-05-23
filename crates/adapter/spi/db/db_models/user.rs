use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_user: i32,
    pub name: String,
    pub phone: String,
    pub address: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::order::Entity",
        from = "Column::IdUser",
        to = "super::order::Column::IdUser"
    )]
    Order,
}

impl ActiveModelBehavior for ActiveModel {}
