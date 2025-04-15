mod models; 

use models::shared::OrderStatus;
use models::order::Order;

fn main() {
    println!("Hello, world!");

    let estado = OrderStatus::Preparing;
    println!("Estado del pedido: {:?}", estado);

    let orden = Order {
        id_order: 1,
        id_user: 10,
        id_delivery_man: 5,
        time: chrono::Local::now().naive_local(),
        state: OrderStatus::Pending,
    };

    println!("Orden: {:?}", orden);
}
