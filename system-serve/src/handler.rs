use super::*;
use actix_web::web;
use bson::{oid::ObjectId, Document};
use std::sync;

pub type CTRL = web::Data<sync::Arc<Controller>>;

pub(crate) mod auth;
pub(crate) mod button;
pub(crate) mod job;
pub(crate) mod menu;
pub(crate) mod org;
pub(crate) mod product;
pub(crate) mod provinces;
pub(crate) mod role;
pub(crate) mod theme;
use crate::service::InitCrud;

pub struct Controller {
    /// 用户服务
    pub user_service: service::UserService,
    /// 角色服务
    pub role_service: service::RoleService,
    /// 机构服务
    pub org_service: service::OrgService,
    /// 职务服务
    pub job_service: service::JobService,
    /// 主题颜色
    pub theme_service: service::ThemeService,
    /// 省份
    pub provinces_service: service::ProvinceService,
    /// 城市
    pub city_service: service::CityService,
    /// 菜单
    pub menu_service: service::MenuService,
    /// 菜单权限
    pub auth_menu_service: service::AuthMenuService,
    /// 用户角色
    pub user_role_service: service::UserRoleService,
    /// 产品
    pub portal_product_service: service::PortalProductService,
    /// 菜单按钮
    pub button_service: service::ButtonService,
}
