use actix_web::web::{self, ServiceConfig};

/* 
use crate::crud_enrollment::infrastructure::controllers::{
    get_schedule_handler,
    withdraw_enrollment_handler,
};

*/

pub fn configure_enrollment_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/deligo")
        
            
    );
}
