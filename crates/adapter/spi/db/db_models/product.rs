use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "product")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id_product : i32,
    pub name : String,
    pub price : Decimal,

}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::order_detail::Entity",
        from = "Column::IdProduct",
        to = "super::order_detail::Column::IdProduct"
    )]
    OrderDetails,
}

impl ActiveModelBehavior for ActiveModel {}
