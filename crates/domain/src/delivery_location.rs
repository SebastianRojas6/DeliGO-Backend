use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "delivery_location")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_location: i32,
    pub id_delivery_man: i32,
    pub latitude: f32,
    pub longitude: f32,
    pub time_delivery_man: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}