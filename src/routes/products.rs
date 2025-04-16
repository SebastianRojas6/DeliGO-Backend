use crate::models::product::{Product, ProductRequest, ProductResponse};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info, warn};
use sqlx::PgPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring product routes...");
    cfg.route("/product/{id}", web::get().to(get_product))
        .route("/product/create", web::post().to(create_product))
        .route("/product/update", web::put().to(update_product))
        .route("/product/delete", web::post().to(delete_product));
    info!("Product routes configured successfully.");
}
async fn create_product(
    pool: web::Data<PgPool>,
    product: web::Json<ProductRequest>,
) -> impl Responder {
    info!("Creating product: {:?}", product);
    let result = sqlx::query_as::<_, Product>(
        "INSERT INTO products (name, price) VALUES ($1, $2) RETURNING id_product, name, price",
    )
    .bind(&product.name)
    .bind(product.price)
    .fetch_one(pool.get_ref())
    .await;

    info!("Product created successfully: {:?}", result);
    match result {
        Ok(new_product) => HttpResponse::Created().json(ProductResponse {
            id_product: new_product.id_product,
            name: new_product.name,
            price: new_product.price,
        }),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn get_product(pool: web::Data<PgPool>, product_id: web::Path<i32>) -> impl Responder {
    info!("Fetching product with ID: {}", product_id);
    let result = sqlx::query_as::<_, Product>(
        "SELECT id_product, name, price FROM products WHERE id_product = $1",
    )
    .bind(product_id.into_inner())
    .fetch_one(pool.get_ref())
    .await;

    info!("Product fetched successfully: {:?}", result);
    match result {
        Ok(product) => HttpResponse::Ok().json(ProductResponse {
            id_product: product.id_product,
            name: product.name,
            price: product.price,
        }),
        Err(e) => HttpResponse::NotFound().json(format!("Product not found: {}", e)),
    }
}

async fn update_product(
    pool: web::Data<PgPool>,
    product_id: web::Path<i32>,
    product: web::Json<ProductRequest>,
) -> impl Responder {
    info!("Updating product with ID: {}", product_id);
    let result = sqlx::query_as::<_, Product>(
        "UPDATE products SET name = $1, price = $2 WHERE id_product = $3 RETURNING id_product, name, price"
    )
        .bind(&product.name)
        .bind(product.price)
        .bind(product_id.into_inner())
        .fetch_one(pool.get_ref())
        .await;

    info!("Product updated successfully: {:?}", result);
    match result {
        Ok(updated_product) => HttpResponse::Ok().json(ProductResponse {
            id_product: updated_product.id_product,
            name: updated_product.name,
            price: updated_product.price,
        }),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Error updating product: {}", e))
        }
    }
}

async fn delete_product(pool: web::Data<PgPool>, product_id: web::Path<i32>) -> impl Responder {
    info!("Deleting product with ID: {}", product_id);
    let result = sqlx::query("DELETE FROM products WHERE id_product = $1")
        .bind(product_id.into_inner())
        .execute(pool.get_ref())
        .await;

    info!("Product deleted successfully: {:?}", result);
    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Error deleting product: {}", e))
        }
    }
}
