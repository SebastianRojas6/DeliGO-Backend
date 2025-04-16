use crate::models::order::{Order, OrderRequest, OrderResponse};
use crate::models::shared::OrderStatus;
use actix_web::{web, HttpResponse, Responder};
use log::info;
use sqlx::PgPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring order routes...");
    cfg.route("/order/{id}", web::get().to(get_order))
        .route("/order/create", web::post().to(create_order))
        .route("/order/{id}", web::put().to(update_order))
        .route("/order/{id}", web::delete().to(delete_order));
    info!("Order routes configured successfully.");
}

async fn create_order(pool: web::Data<PgPool>, order: web::Json<OrderRequest>) -> impl Responder {
    info!("Creating order: {:?}", order);
    let new_order = Order {
        id_order: 0,
        id_user: order.user_id,
        id_delivery_man: 0,
        time: chrono::Utc::now().naive_utc(),
        state: OrderStatus::Pending,
    };

    let saved_order = sqlx::query_as::<_, Order>(
        "INSERT INTO orders (id_user, id_delivery_man, time, state) VALUES ($1, $2, $3, $4) RETURNING id_order, id_user, id_delivery_man, time, state"
    )
        .bind(new_order.id_user)
        .bind(new_order.id_delivery_man)
        .bind(new_order.time)
        .bind(new_order.state)
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| HttpResponse::InternalServerError().json(format!("Error: {}", e)))?;

    HttpResponse::Created().json(OrderResponse {
        id: saved_order.id_order,
        status: saved_order.state,
        estimated_time: "30 minutes".to_string(),
    })
}

async fn get_order(pool: web::Data<PgPool>, order_id: web::Path<i32>) -> impl Responder {
    info!("Fetching order with ID: {}", order_id);
    let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id_order = $1")
        .bind(order_id.into_inner())
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| HttpResponse::InternalServerError().json(format!("Error: {}", e)))?;

    HttpResponse::Ok().json(OrderResponse {
        id: order.id_order,
        status: order.state,
        estimated_time: "30 minutes".to_string(), // Este es solo un ejemplo
    })
}

async fn update_order(
    pool: web::Data<PgPool>,
    order_id: web::Path<i32>,
    order: web::Json<OrderRequest>,
) -> impl Responder {
    info!("Updating order with ID: {}", order_id);
    let updated_order = sqlx::query_as::<_, Order>(
        "UPDATE orders SET id_user = $1, id_delivery_man = $2, time = $3, state = $4 WHERE id_order = $5 RETURNING id_order, id_user, id_delivery_man, time, state"
    )
        .bind(order.user_id)
        .bind(0)
        .bind(chrono::Utc::now().naive_utc())
        .bind(OrderStatus::Pending)
        .bind(order_id.into_inner())
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| HttpResponse::InternalServerError().json(format!("Error: {}", e)))?;

    HttpResponse::Ok().json(OrderResponse {
        id: updated_order.id_order,
        status: updated_order.state,
        estimated_time: "30 minutes".to_string(),
    })
}

async fn delete_order(pool: web::Data<PgPool>, order_id: web::Path<i32>) -> impl Responder {
    info!("Deleting order with ID: {}", order_id);
    sqlx::query("DELETE FROM orders WHERE id_order = $1")
        .bind(order_id.into_inner())
        .execute(pool.get_ref())
        .await
        .map_err(|e| HttpResponse::InternalServerError().json(format!("Error: {}", e)))?;

    HttpResponse::NoContent().finish()
}
