use std::str::FromStr;
use actix_web::{web, HttpResponse, Error};
use bigdecimal::{BigDecimal, ToPrimitive};
use sqlx::PgPool;
use crate::models::product::{Product, ProductRequest, ProductResponse};

#[derive(Debug)]
pub struct ProductController;

impl ProductController {
    pub async fn create_product(
        pool: web::Data<PgPool>,
        product_req: web::Json<ProductRequest>,
    ) -> Result<HttpResponse, Error> {
        let product_req = product_req.into_inner();

        // let price = Decimal::new((product_req.price * 100.0) as i64, 2);
        let price_amount = BigDecimal::from_str(&product_req.price.to_string()).unwrap();

        let product = sqlx::query_as!(
            Product,
            r#"
            INSERT INTO products (name, price)
            VALUES ($1, $2)
            RETURNING id_product, name, price
            "#,
            product_req.name,
            price_amount
        )
            .fetch_one(&**pool)
            .await;

        match product {
            Ok(product) => Ok(HttpResponse::Created().json(ProductResponse {
                id_product: product.id_product,
                name: product.name,
                price: BigDecimal::to_f32(&product.price).unwrap(),
            })),
            Err(e) => {
                // Handle the sqlx::Error and return a custom response
                match e {
                    sqlx::Error::Database(db_error) => {
                        // Handle database-specific errors (e.g., constraints violations, etc.)
                        Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", db_error)))
                    }
                    sqlx::Error::RowNotFound => {
                        // Handle the case when the query returns no rows (e.g., invalid product)
                        Ok(HttpResponse::NotFound().body("Product not found"))
                    }
                    _ => {
                        // Handle other types of errors (e.g., connection issues)
                        Ok(HttpResponse::InternalServerError().body(format!("Unexpected error: {:?}", e)))
                    }
                }
            }
        }
    }

    pub async fn get_all_products(
        pool: web::Data<PgPool>,
    ) -> Result<HttpResponse, Error> {
        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT id_product, name, price
            FROM products
            "#
        )
            .fetch_all(&**pool)
            .await;

        match products {
            Ok(products) => Ok(HttpResponse::Ok().json(products)),
            Err(e) => match e {
                sqlx::Error::Database(db_error) => Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", db_error))),
                _ => Ok(HttpResponse::InternalServerError().body(format!("Unexpected error: {:?}", e))),
            }
        }
    }

    pub async fn get_product_by_id(
        pool: web::Data<PgPool>,
        product_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT id_product, name, price
            FROM products
            WHERE id_product = $1
            "#,
            product_id.into_inner()
        )
            .fetch_optional(&**pool)
            .await;

        match product {
            Ok(Some(product)) => Ok(HttpResponse::Ok().json(product)),
            Ok(None) => Ok(HttpResponse::NotFound().body("Product not found")),
            Err(e) => match e {
                sqlx::Error::Database(db_error) => Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", db_error))),
                _ => Ok(HttpResponse::InternalServerError().body(format!("Unexpected error: {:?}", e))),
            }
        }
    }

    pub async fn update_product(
        pool: web::Data<PgPool>,
        product_id: web::Path<i32>,
        product_req: web::Json<ProductRequest>,
    ) -> Result<HttpResponse, Error> {
        let product_req = product_req.into_inner();

        let price_amount = BigDecimal::from_str(&product_req.price.to_string()).unwrap();

        let updated_product = sqlx::query_as!(
            Product,
            r#"
            UPDATE products
            SET name = $1, price = $2
            WHERE id_product = $3
            RETURNING id_product, name, price
            "#,
            product_req.name,
            price_amount,
            product_id.into_inner()
        )
            .fetch_optional(&**pool)
            .await;

        match updated_product {
            Ok(Some(product)) => Ok(HttpResponse::Ok().json(ProductResponse {
                id_product: product.id_product,
                name: product.name,
                price: BigDecimal::to_f32(&product.price).unwrap(),
            })),
            Ok(None) => Ok(HttpResponse::NotFound().body("Product not found")),
            Err(e) => match e {
                sqlx::Error::Database(db_error) => Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", db_error))),
                _ => Ok(HttpResponse::InternalServerError().body(format!("Unexpected error: {:?}", e))),
            }
        }
    }

    pub async fn delete_product(
        pool: web::Data<PgPool>,
        product_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let deleted_count = sqlx::query!(
            r#"
            DELETE FROM products
            WHERE id_product = $1
            "#,
            product_id.into_inner()
        )
            .execute(&**pool)
            .await;

        match deleted_count {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    Ok(HttpResponse::NotFound().body("Product not found"))
                } else {
                    Ok(HttpResponse::NoContent().finish())
                }
            }
            Err(e) => match e {
                sqlx::Error::Database(db_error) => Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", db_error))),
                _ => Ok(HttpResponse::InternalServerError().body(format!("Unexpected error: {:?}", e))),
            }
        }

    }
}
