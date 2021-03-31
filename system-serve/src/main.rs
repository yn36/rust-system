#[macro_use]
mod config;
use config::toml_conf;

mod handler;
mod model;
mod service;

// #[macro_use]
extern crate redis;
extern crate yn_util;
use crate::yn_util::dao;
use crate::yn_util::utils::*;
use actix_web::{middleware, web, App, HttpServer};
use bson::{doc, ser};
use lazy_static::*;
use log::*;
use model::*;
use std::sync;

#[macro_use]
extern crate anyhow;

fn init_ctrl() -> sync::Arc<handler::Controller> {
    let user_service = service::UserService::new();
    let role_service = service::RoleService::new();
    let org_service = service::OrgService::new();
    let job_service = service::JobService::new();
    let provinces_service = service::ProvinceService::new();
    let city_service = service::CityService::new();
    let theme_service = service::ThemeService::new();
    let menu_service = service::MenuService::new();
    let user_role_service = service::UserRoleService::new();
    let portal_product_service = service::PortalProductService::new();
    let auth_menu_service = service::AuthMenuService::new();
    let button_service = service::ButtonService::new();
    let ctrl = handler::Controller {
        user_service,
        role_service,
        org_service,
        job_service,
        provinces_service,
        city_service,
        theme_service,
        menu_service,
        user_role_service,
        portal_product_service,
        auth_menu_service,
        button_service,
    };
    let ctrl = sync::Arc::new(ctrl);
    ctrl
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    config::toml_conf::init_logger();
    let host = &*toml_conf::SETTING.app.host;
    let port = &toml_conf::SETTING.app.port.clone();
    // 地址
    let addr = format!("{}:{}", host, port);
    // 线程
    let workers = &toml_conf::SETTING.app.workers.clone();

    // 初始化缓存服务器
    // toml_conf::init_caches();
    // let redis_cli = toml_conf::redis_client();
    // let redis_cli = sync::Arc::new(redis_cli);

    // 初始化数据库服务器
    dao::init_dbs(&toml_conf::get_conn_string()).await;

    let ctrl = init_ctrl();

    HttpServer::new(move || {
        App::new()
            .data(ctrl.clone())
            // .data(redis_cli.clone())
            .wrap(middleware::Logger::default())
            .app_data(<web::Json::<request::QueryBody<user::UserModel>> as actix_web::FromRequest>::configure(
                |cfg| {
                    cfg.error_handler(|err, req| {
                        info!("参数错误 = {:?}",req);
                        let mess = format!("path={}, {}", req.uri(), err);
                        log::error!("参数错误, path={}, {}", req.uri(), err);
                        BusinessError::ArgumentError {source:anyhow!(mess)}.into()
                    })
                },
            ))
            .service(
                web::scope("/system")
                    .configure(handler::auth::route)
                    .configure(handler::role::route)
                    .configure(handler::org::route)
                    .configure(handler::job::route)
                    .configure(handler::provinces::route)
                    .configure(handler::theme::route)
                    .configure(handler::product::route)
                    .configure(handler::button::route)
                    .configure(handler::menu::route),
            )
    })
    .bind(addr)?
    .workers(*workers)
    .run()
    .await
}
