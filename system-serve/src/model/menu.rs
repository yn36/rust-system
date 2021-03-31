//！ 菜单模型

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuModel {
    #[serde(default)]
    pub _id: Option<String>,

    /// 产品id
    #[serde(default)]
    pub prod_id: Option<String>,

    /// 菜单标题
    #[serde(default)]
    pub title: Option<String>,

    /// 菜单名称
    #[serde(default)]
    pub name: Option<String>,

    /// 菜单别名
    #[serde(default)]
    pub alias: Option<String>,

    /// 菜单url
    #[serde(default)]
    pub url: Option<String>,

    /// 菜单组件路径
    #[serde(default)]
    pub component: Option<String>,

    /// 菜单配置信息
    #[serde(default)]
    pub config: Option<String>,

    /// 父节点id
    #[serde(default)]
    pub parent_id: Option<String>,

    /// 排序
    #[serde(default)]
    pub sort: Option<i32>,

    /// 描述
    #[serde(default)]
    pub desc: Option<String>,

    /// 按钮
    #[serde(default)]
    pub btns: Option<String>,

    /// 是否子节点
    #[serde(default)]
    pub is_leaf: Option<bool>,

    /// 是否隐藏
    #[serde(default)]
    pub is_hidden: Option<bool>,

    /// 是否 iframe
    #[serde(default)]
    pub is_iframe: Option<bool>,

    /// 是否外部链接
    #[serde(default)]
    pub is_external: Option<bool>,

    /// 小图标
    #[serde(default)]
    pub pic_min: Option<String>,

    /// 中图标
    #[serde(default)]
    pub pic_mid: Option<String>,

    /// 大图标
    #[serde(default)]
    pub pic_max: Option<String>,

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

impl Default for MenuModel {
    fn default() -> Self {
        Self {
            _id: None,
            prod_id: None,
            title: None,
            name: None,
            alias: None,
            url: None,
            component: None,
            config: None,
            parent_id: None,
            sort: None,
            desc: None,
            btns: None,
            is_leaf: None,
            is_hidden: None,
            is_iframe: None,
            is_external: None,
            pic_min: None,
            pic_mid: None,
            pic_max: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
        }
    }
}

impl MenuModel {
    /// 数据库名字
    #[allow(dead_code)]
    pub const DATA_BASE_NAME: &'static str = "YNOS";

    /// 集合名字
    #[allow(dead_code)]
    pub const COLL_NAME: &'static str = "menu_list";
}

/// 菜单权限
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthMenuModel {
    #[serde(default)]
    pub _id: Option<String>,

    /// 菜单名称
    #[serde(default)]
    pub name: Option<String>,

    /// 菜单id
    #[serde(default)]
    pub menu_id: Option<String>,

    /// 用户id
    #[serde(default)]
    pub user_id: Option<String>,

    /// 角色id
    #[serde(default)]
    pub role_id: Option<String>,

    /// 关联id user_id or role_id
    #[serde(default)]
    pub relation_id: Option<String>,

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

impl Default for AuthMenuModel {
    fn default() -> Self {
        Self {
            _id: None,
            name: None,
            menu_id: None,
            user_id: None,
            role_id: None,
            relation_id: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
        }
    }
}

impl AuthMenuModel {
    /// 数据库名字
    #[allow(dead_code)]
    pub const DATA_BASE_NAME: &'static str = "YNOS";

    /// 集合名字
    #[allow(dead_code)]
    pub const COLL_NAME: &'static str = "auth_menu_list";
}
