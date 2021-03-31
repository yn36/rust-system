//！ 角色模型

use super::*;

/// 角色
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RoleModel {
    #[serde(default)]
    pub _id: Option<String>,

    /// 角色名
    #[serde(default)]
    pub name: Option<String>,

    /// 角色编码
    #[serde(default)]
    pub role_code: Option<String>,

    /// 描述
    #[serde(default)]
    pub desc: Option<String>,

    /// 创建人
    #[serde(default)]
    pub create_by: Option<String>,

    /// 创建时间
    #[serde(default)]
    pub create_time: Option<String>,

    /// 修改人
    #[serde(default)]
    pub update_by: Option<String>,

    /// 修改时间
    #[serde(default)]
    pub update_time: Option<String>,
}

/// 用户角色
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRoleModel {
    #[serde(default)]
    pub _id: Option<String>,

    /// 用户 id
    #[serde(default)]
    pub user_id: Option<String>,

    /// 角色 id
    #[serde(default)]
    pub role_id: Option<String>,

    /// 角色名称
    #[serde(default)]
    pub name: Option<String>,

    /// 创建人
    #[serde(default)]
    pub create_by: Option<String>,

    /// 创建时间
    #[serde(default)]
    pub create_time: Option<String>,
}

impl Default for RoleModel {
    fn default() -> Self {
        Self {
            _id: None,
            name: None,
            role_code: None,
            desc: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
        }
    }
}

impl Default for UserRoleModel {
    fn default() -> Self {
        Self {
            _id: None,
            user_id: None,
            role_id: None,
            name: None,
            create_by: None,
            create_time: None,
        }
    }
}

impl RoleModel {
    /// 数据库名字
    #[allow(dead_code)]
    pub const DATA_BASE_NAME: &'static str = "YNOS";

    /// 集合名字
    #[allow(dead_code)]
    pub const COLL_NAME: &'static str = "role_list";
}

impl UserRoleModel {
    /// 数据库名字
    #[allow(dead_code)]
    pub const DATA_BASE_NAME: &'static str = "YNOS";

    /// 集合名字
    #[allow(dead_code)]
    pub const COLL_NAME: &'static str = "user_roles";
}
