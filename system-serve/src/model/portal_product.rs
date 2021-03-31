//! 门户产品

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortalProductModel {
    #[serde(default)]
    pub _id: Option<String>,

    /// 产品名称
    #[serde(default)]
    pub name: Option<String>,

    /// 产品编码
    #[serde(default)]
    pub codes: Option<String>,

    /// 欢迎页URL
    #[serde(default)]
    pub welcome: Option<String>,

    /// 版本号
    #[serde(default)]
    pub vers_no: Option<String>,

    /// 版本说明
    #[serde(default)]
    pub vers_desc: Option<String>,

    /// 产品描述
    #[serde(default)]
    pub prod_desc: Option<String>,

    /// 功能说明
    #[serde(default)]
    pub sfc_desc: Option<String>,

    /// 适用行业
    #[serde(default)]
    pub inds_desc: Option<String>,

    /// 所在机构
    #[serde(default)]
    pub org_id: Option<String>,

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

impl Default for PortalProductModel {
    fn default() -> Self {
        Self {
            _id: None,
            name: None,
            codes: None,
            welcome: None,
            vers_no: None,
            vers_desc: None,
            prod_desc: None,
            sfc_desc: None,
            inds_desc: None,
            org_id: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
        }
    }
}

impl PortalProductModel {
    /// 数据库名字
    #[allow(dead_code)]
    pub const DATA_BASE_NAME: &'static str = "YNOS";

    /// 集合名字
    #[allow(dead_code)]
    pub const COLL_NAME: &'static str = "product_list";
}
