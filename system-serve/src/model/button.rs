//！ 按钮模型

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonModel {
    #[serde(default)]
    pub _id: Option<String>,

    /// 产品id
    #[serde(default)]
    pub prod_id: Option<String>,

    /// 菜单id
    #[serde(default)]
    pub menu_id: Option<String>,

    /// 按钮编码
    #[serde(default)]
    pub btn_code: Option<String>,

    /// 按钮标题
    #[serde(default)]
    pub title: Option<String>,

    /// 按钮名称
    #[serde(default)]
    pub name: Option<String>,

    /// 按钮别名
    #[serde(default)]
    pub alias: Option<String>,

    /// 按钮配置信息
    #[serde(default)]
    pub config: Option<String>,

    /// 状态
    #[serde(default)]
    pub status: Option<i32>,

    /// 排序
    #[serde(default)]
    pub sort: Option<i32>,

    /// 描述
    #[serde(default)]
    pub desc: Option<String>,

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

impl Default for ButtonModel {
    fn default() -> Self {
        Self {
            _id: None,
            prod_id: None,
            btn_code: None,
            title: None,
            name: None,
            alias: None,
            sort: None,
            menu_id: None,
            config: None,
            status: None,
            desc: None,
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

impl ButtonModel {
    /// 数据库名字
    #[allow(dead_code)]
    pub const DATA_BASE_NAME: &'static str = "YNOS";

    /// 集合名字
    #[allow(dead_code)]
    pub const COLL_NAME: &'static str = "button_list";
}
