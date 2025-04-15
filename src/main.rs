mod models; 

use models::shared::OrderStatus;
use models::order::OrderRequest;

fn main() {

    let estado = OrderStatus::Preparing;
    
    println!("Estado del pedido: {:?}", estado);

    let solicitud = OrderRequest {
        user_id: 10,
        items: vec![1, 2, 3],
        delivery_address: String::from("Av. Miguelcmep 123"),
    };
    
    
    println!("Usuario ID: {}", solicitud.user_id);
    println!("Items: {:?}", solicitud.items);
    println!("Dirección: {}", solicitud.delivery_address);


}
