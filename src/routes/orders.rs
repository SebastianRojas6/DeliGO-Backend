use actix_web::{web, HttpResponse, Error};
use sqlx::PgPool;
use crate::models::shared::OrderStatus;
use crate::models::order::{Order, OrderRequest, OrderResponse};
use log::info;

#[derive(Debug)]
pub struct OrderController;

impl OrderController {
    pub async fn create_order(
        pool: web::Data<PgPool>,
        order_req: web::Json<OrderRequest>,
    ) -> Result<HttpResponse, Error> {
        let order_req = order_req.into_inner();

        let order = sqlx::query_as!(
            Order,
            r#"
            INSERT INTO orders (id_user, id_delivery_man, time, state, delivery_address)
            VALUES ($1, $2, CURRENT_TIMESTAMP, 'pending', $3)
            RETURNING id_order, id_user, id_delivery_man, time, state AS "state: _", delivery_address
            "#,
            order_req.user_id,
            1,
            order_req.delivery_address
        )
            .fetch_one(&**pool)
            .await;

        match order {
            Ok(order) => {
                let order_response = OrderResponse {
                    id: order.id_order,
                    status: order.state,
                    estimated_time: "30 minutes".to_string(), // Placeholder for estimated time
                };
                Ok(HttpResponse::Created().json(order_response))
            }
            Err(e) => {
                // Handle the sqlx::Error and return a custom response
                match e {
                    sqlx::Error::Database(db_error) => {
                        // Handle database-specific errors (e.g., constraints violations, etc.)
                        Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", db_error)))
                    }
                    sqlx::Error::RowNotFound => {
                        // Handle the case when the query returns no rows (e.g., invalid product)
                        Ok(HttpResponse::NotFound().body("Order not found"))
                    }
                    _ => {
                        // Handle other types of errors (e.g., connection issues)
                        Ok(HttpResponse::InternalServerError().body(format!("Unexpected error: {:?}", e)))
                    }
                }
            }
        }
    }

    pub async fn get_all_orders(
        pool: web::Data<PgPool>,
    ) -> Result<HttpResponse, Error> {
        let orders = sqlx::query_as!(
            Order,
            r#"
            SELECT id_order, id_user, id_delivery_man, delivery_address, time, state AS "state: _"
            FROM orders
            "#
        )
            .fetch_all(&**pool)
            .await;

        match orders {
            Ok(orders) => Ok(HttpResponse::Ok().json(orders)),
            Err(e) => match e {
                sqlx::Error::Database(db_error) => Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", db_error))),
                sqlx::Error::RowNotFound => Ok(HttpResponse::NotFound().body("No orders found")),
                _ => Ok(HttpResponse::InternalServerError().body(format!("Unexpected error: {:?}", e))),
            }
        }
    }

    pub async fn get_order_by_id(
        pool: web::Data<PgPool>,
        order_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let order = sqlx::query_as!(
            Order,
            r#"
            SELECT id_order, id_user, id_delivery_man, delivery_address, time, state AS "state: _"
            FROM orders
            WHERE id_order = $1
            "#,
            order_id.into_inner()
        )
            .fetch_optional(&**pool)
            .await;

        match order {
            Ok(Some(order)) => Ok(HttpResponse::Ok().json(order)),
            Ok(None) => Ok(HttpResponse::NotFound().body("Order not found")),
            Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Unexpected error: {:?}", e))),
        }
    }

    // Actualizar el estado de la orden
    pub async fn update_order_status(
        pool: web::Data<PgPool>,
        order_id: web::Path<i32>,
        status: web::Json<OrderStatus>,
    ) -> Result<HttpResponse, Error> {
        let new_status = status.into_inner();

        let updated_order = sqlx::query_as!(
            Order,
            r#"
            UPDATE orders
            SET state = $1
            WHERE id_order = $2
            RETURNING id_order, id_user, id_delivery_man, delivery_address, time, state AS "state: _"
            "#,
            new_status as OrderStatus,
            order_id.into_inner()
        )
            .fetch_optional(&**pool)
            .await;

        match updated_order {
            Ok(Some(order)) => Ok(HttpResponse::Ok().json(order)),
            Ok(None) => Ok(HttpResponse::NotFound().body("Order not found")),
            Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Unexpected error: {:?}", e))),
        }
    }

    // Eliminar una orden
    pub async fn delete_order(
        pool: web::Data<PgPool>,
        order_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let deleted_count = sqlx::query!(
            r#"
            DELETE FROM orders
            WHERE id_order = $1
            "#,
            order_id.into_inner()
        )
            .execute(&**pool)
            .await;

        match deleted_count {
            Ok(count) if count.rows_affected() > 0 => Ok(HttpResponse::Ok().body("Order deleted")),
            Ok(_) => Ok(HttpResponse::NotFound().body("Order not found")),
            Err(e) => match e {
                sqlx::Error::Database(db_error) => Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", db_error))),
                _ => Ok(HttpResponse::InternalServerError().body(format!("Unexpected error: {:?}", e))),
            }
        }
    }
}
