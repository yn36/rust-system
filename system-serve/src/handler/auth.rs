use super::*;
pub mod controller;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(controller::reg_user)
            .service(controller::get_user_list)
            .service(controller::login)
            .service(controller::update_user)
            .service(controller::delete_user)
            .service(controller::logout)
            .service(controller::is_authed)
            .service(controller::get_user_roles_list)
            .service(controller::get_prods_menu_list)
            .service(controller::save_auth_menu)
            .service(controller::get_prods_menu)
            .service(controller::save_roles),
    );
}
