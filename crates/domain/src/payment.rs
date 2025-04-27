use crate::order;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "payment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_payment: i32,
    pub id_order: i32,
    pub total_amount: f64,
    pub payment_method: PaymentMethod,
    pub payment_status: PaymentStatus,
    pub payment_date: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::order::Entity",
        from = "Column::IdOrder",
        to = "super::order::Column::IdOrder"
    )]
    Order,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(20))")]
pub enum PaymentMethod {
    #[sea_orm(string_value = "Visa")]
    Visa,
    #[sea_orm(string_value = "MasterCard")]
    MasterCard,
    #[sea_orm(string_value = "Yape")]
    Yape,
    #[sea_orm(string_value = "Plin")]
    Plin,
    #[sea_orm(string_value = "Paypal")]
    Paypal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(20))")]
pub enum PaymentStatus {
    #[sea_orm(string_value = "Pending")]
    Pending,

    #[sea_orm(string_value = "Paid")]
    Paid,

    #[sea_orm(string_value = "Cancelled")]
    Cancelled,
}

impl Related<order::Entity> for Entity {
    fn to() -> RelationDef {
        order::Relation::Payment.def()
    }
}