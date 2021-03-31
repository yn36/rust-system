use super::*;
pub mod controller;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/provinces")
            .service(controller::find_provinces)
            .service(controller::find_city),
    );
}
