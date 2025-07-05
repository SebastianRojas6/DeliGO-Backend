use crate::domain::models::product_model::Product;
use crate::domain::repository::ProductRepository;
use crate::infrastructure::entity::product;
use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};
use shared::connect_to_supabase;

pub struct ProductQuery {
    pub db: DatabaseConnection,
}

impl ProductQuery {
    pub async fn new() -> Result<Self, String> {
        let db = connect_to_supabase().await.map_err(|e| e.to_string())?;
        Ok(Self { db })
    }
}

#[async_trait]
impl ProductRepository for ProductQuery {
    async fn get_all(&self) -> Result<Vec<Product>, String> {
        product::Entity::find().all(&self.db).await.map_err(|e| e.to_string()).map(|products| {
            products
                .into_iter()
                .map(|p| Product {
                    id: p.id_product,
                    name: p.name.unwrap_or_default(),
                    price: p.price.map(|d| d.to_string()).unwrap_or_default(),
                })
                .collect()
        })
    }

    async fn get_by_purchase_for_user(&self, user_id: i32, product_id: i32) -> Result<Vec<Product>, String> {
        use crate::infrastructure::entity::{order, order_details, payment};

        let products = product::Entity::find()
            .join(JoinType::InnerJoin, product::Relation::OrderDetails.def())
            .join(JoinType::InnerJoin, order_details::Relation::Order.def())
            .join(JoinType::InnerJoin, order::Relation::Payment.def())
            .filter(product::Column::IdProduct.eq(product_id))
            .filter(order::Column::IdUser.eq(user_id))
            .filter(payment::Column::PaymentStatus.eq("completed"))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(products
            .into_iter()
            .map(|p| Product {
                id: p.id_product,
                name: p.name.unwrap_or_default(),
                price: p.price.map(|d| d.to_string()).unwrap_or_default(),
            })
            .collect())
    }

    async fn get_selected_products(&self, user_id: i32) -> Result<Vec<Product>, String> {
        use crate::infrastructure::entity::shopping_cart;

        let products = product::Entity::find()
            .join(JoinType::InnerJoin, product::Relation::ShoppingCart.def())
            .filter(shopping_cart::Column::IdUser.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(products
            .into_iter()
            .map(|p| Product {
                id: p.id_product,
                name: p.name.unwrap_or_default(),
                price: p.price.map(|d| d.to_string()).unwrap_or_default(),
            })
            .collect())
    }
}
