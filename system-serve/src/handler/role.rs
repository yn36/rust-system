use super::*;
pub mod controller;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/role")
            .service(controller::save)
            .service(controller::delete_role)
            .service(controller::update_role)
            .service(controller::find_all)
            .service(controller::find_roles),
    );
}
