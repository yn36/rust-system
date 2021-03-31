use super::*;
pub mod controller;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/job")
            .service(controller::save)
            .service(controller::delete)
            .service(controller::update)
            .service(controller::find),
    );
}
