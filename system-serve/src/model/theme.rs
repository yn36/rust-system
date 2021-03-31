//！ 主题模型

use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThemeModel {
    #[serde(default)]
    pub _id: Option<String>,

    /// 外观  auto:自动  light:光模式  night:暗夜模式
    #[serde(default)]
    pub appearance: Option<String>,

    /// 主题色 只接受 16进制的色号 如 #000000
    #[serde(default)]
    pub theme_color: Option<String>,

    /// 导航设置 side:侧面 head:头部
    #[serde(default)]
    pub navigate: Option<String>,

    /// 是否固定导航 1.true 0.false
    #[serde(default)]
    pub fixed_header: Option<bool>,

    /// 色弱模式  1.true 0.false
    #[serde(default)]
    pub week_mode: Option<bool>,

    /// 多标签模式  1.true 0.false
    #[serde(default)]
    pub multi_pages: Option<bool>,

    /// 用户id
    #[serde(default)]
    pub user_id: Option<ObjectId>,

    /// 用户name
    #[serde(default)]
    pub user_name: Option<String>,

    /// 创建人
    #[serde(default)]
    pub create_by: Option<ObjectId>,

    /// 创建时间
    #[serde(default)]
    pub create_time: Option<String>,

    /// 修改人
    #[serde(default)]
    pub update_by: Option<ObjectId>,

    /// 修改时间
    #[serde(default)]
    pub update_time: Option<String>,
}

impl Default for ThemeModel {
    fn default() -> Self {
        Self {
            _id: None,
            appearance: None,
            theme_color: None,
            navigate: None,
            fixed_header: None,
            week_mode: None,
            multi_pages: None,
            user_id: None,
            user_name: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
        }
    }
}

impl ThemeModel {
    /// 数据库名字
    #[allow(dead_code)]
    pub const DATA_BASE_NAME: &'static str = "YNOS";

    /// 集合名字
    #[allow(dead_code)]
    pub const COLL_NAME: &'static str = "theme_list";
}
