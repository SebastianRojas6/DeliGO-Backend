use actix_web::web;

use super::controller::{
    product_controller::{get_all_products, get_by_purchase_for_user, get_selected_products},
    user_controller::{change_order_status_controller, rate_delivery_controller},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/rate-delivery").route(web::post().to(rate_delivery_controller)))
        .service(web::resource("/products").route(web::get().to(get_all_products)))
        .service(web::resource("/products/purchase/{user_id}/{product_id}").route(web::get().to(get_by_purchase_for_user)))
        .service(web::resource("/products/selected/{user_id}").route(web::get().to(get_selected_products)))
        .service(web::resource("/orders/change-status").route(web::put().to(change_order_status_controller)));
}
