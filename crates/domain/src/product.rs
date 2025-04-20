use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "product")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id_product : i32,
    pub name : String,
    pub price : BigDecimal,

}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// Api rest 

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductRequest {
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id_product: i32,
    pub name: String,
    pub price: f32,
}

// --- Conversión (trabjando) --