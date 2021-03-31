use super::*;
pub mod controller;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/theme")
            .service(controller::save)
            .service(controller::find_theme)
            .service(controller::update), // .service(controller::find_roles),
    );
}
