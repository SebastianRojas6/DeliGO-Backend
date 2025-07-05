use crate::domain::models::product_model::Product;
use crate::domain::repository::ProductRepository;
// Es el modelo de como estan las entidades en la base de datos
use crate::infrastructure::entity::product;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbBackend, EntityTrait, Statement};
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
        // Verify user purchased this specific product
        let stmt = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            SELECT p.id_product, p.name, p.price 
            FROM product p
            WHERE p.id_product = $1
            AND EXISTS (
                SELECT 1 FROM "order" o
                JOIN order_details od ON o.id_order = od.id_order
                JOIN payment pay ON o.id_order = pay.id_order
                WHERE o.id_user = $2
                AND od.id_product = $1
                AND pay.payment_status = 'completed'  -- Assuming successful payment
            )"#,
            vec![product_id.into(), user_id.into()],
        );

        product::Entity::find().from_raw_sql(stmt).all(&self.db).await.map_err(|e| e.to_string()).map(|products| {
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

    async fn get_selected_products(&self, user_id: i32) -> Result<Vec<Product>, String> {
        let stmt = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
        SELECT DISTINCT p.id_product, p.name, p.price
        FROM product p
        INNER JOIN shopping_cart sc ON p.id_product = sc.id_product
        WHERE sc.id_user = $1
        "#,
            vec![user_id.into()],
        );

        product::Entity::find().from_raw_sql(stmt).all(&self.db).await.map_err(|e| e.to_string()).map(|products| {
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
}
