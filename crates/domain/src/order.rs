use sea_orm::entity::prelude::*;
use crate::shared::OrderStatus;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_order: i32,
    pub id_user: i32,
    pub id_delivery_man: i32,
    pub time: DateTimeWithTimeZone, 
    pub state: OrderStatus,
    pub delivery_address: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::IdUser",
        to = "super::user::Column::IdUser"
    )]
    User,

    #[sea_orm(
        belongs_to = "super::delivery_man::Entity",
        from = "Column::IdDeliveryMan",
        to = "super::delivery_man::Column::IdDeliveryMan"
    )]
    DeliveryMan,

    #[sea_orm(
        has_many = "super::order_detail::Entity",
        from = "Column::IdOrder",
        to = "super::order_detail::Column::IdOrder"
    )]
    OrderDetail,

    #[sea_orm(
        has_one = "super::payment::Entity",
        from = "Column::IdOrder",
        to = "super::payment::Column::IdOrder"
    )]
    Payment,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::delivery_man::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DeliveryMan.def()
    }
}

impl Related<super::order_detail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrderDetail.def()
    }
}

impl Related<super::payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
