use super::*;
use bson::Document;
use dao::Dao;

/// 初始化 增删改查 trait
#[async_trait::async_trait]
pub trait InitCrud {
    fn get_op(&self) -> &dao::Dao;

    /// 数据保存
    async fn save(&self, data: Document) -> Result<Document, BusinessError> {
        let oid = self.get_op().save(&data).await.unwrap();
        let result = self.get_op().find_by_id(oid).await.unwrap();
        match result {
            Some(data) => Ok(data),
            None => {
                return Err(BusinessError::InternalError {
                    source: anyhow!("错误"),
                });
            }
        }
    }

    /// 查询列表
    async fn find(
        &self,
        filter: Document,
        page: Option<i64>,
        limit: Option<i64>,
        sort_name: Option<String>,
        sort_order: Option<String>,
        is_all: bool,
        oid_type: Option<Vec<&str>>,
    ) -> Result<(Vec<Document>, i64), BusinessError> {
        let list: Vec<Document> = match self
            .get_op()
            .find(
                filter.clone(),
                limit,
                page,
                sort_name,
                sort_order,
                is_all,
                oid_type,
            )
            .await
        {
            Ok(list) => list,
            Err(e) => Err(BusinessError::InternalError { source: anyhow!(e) })?,
        };
        let count = self.get_op().count(filter.clone()).await?;

        Ok((list, count))
    }

    /// 根据id 查询
    #[allow(dead_code)]
    async fn find_by_id(&self, id: bson::oid::ObjectId) -> Result<Option<Document>, BusinessError> {
        self.get_op().find_by_id(id).await
    }

    /// 根据条件查询一条
    #[allow(dead_code)]
    async fn find_one(&self, filter: Document) -> Result<Option<Document>, BusinessError> {
        self.get_op().find_one(filter).await
    }

    /// 数据更新
    #[allow(dead_code)]
    async fn update(&self, data: Document) -> Result<Option<Document>, BusinessError> {
        self.get_op().update(data).await
    }

    /// 数据删除
    #[allow(dead_code)]
    async fn remove(&self, ids: String) -> Result<i64, BusinessError> {
        self.get_op().remove(ids).await
    }
}
/// 菜单
mod button_service;
/// 城市
mod city_service;
/// 职务
mod job_service;
/// 菜单
mod menu_service;
/// 机构
mod org_service;
/// 产品
mod portal_product_service;
/// 省份
mod provinces_service;
/// 角色
mod role_service;
/// 主题
mod theme_service;
/// 用户
mod user_service;

pub use button_service::ButtonService;
pub use city_service::CityService;
pub use job_service::JobService;
pub use menu_service::{AuthMenuService, MenuService};
pub use org_service::OrgService;
pub use portal_product_service::PortalProductService;
pub use provinces_service::ProvinceService;
pub use role_service::{RoleService, UserRoleService};
pub use theme_service::ThemeService;
pub use user_service::UserService;
