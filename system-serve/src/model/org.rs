//！ 机构模型

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrgModel {
    #[serde(default)]
    pub _id: Option<String>,

    /// 机构名称
    #[serde(default)]
    pub name: Option<String>,

    /// 机构别名
    #[serde(default)]
    pub alias: Option<String>,

    /// 机构省份 id
    #[serde(default)]
    pub province_id: Option<String>,

    /// 机构省份 name
    #[serde(default)]
    pub province: Option<String>,

    /// 机构城市 id
    #[serde(default)]
    pub city_id: Option<String>,

    /// 机构城市 name
    #[serde(default)]
    pub city: Option<String>,

    /// 机构地址
    #[serde(default)]
    pub addr: Option<String>,

    /// 机构类型 1.企业  2.机构  3.部门
    pub types: Option<i32>,

    /// 机构编码
    pub org_code: Option<String>,

    /// 描述
    #[serde(default)]
    pub desc: Option<String>,

    /// 联系电话
    #[serde(default)]
    pub tel: Option<String>,

    /// 父节点id
    #[serde(default)]
    pub parent_id: Option<String>,

    /// 是否子节点 1.是 0.否
    #[serde(default)]
    pub is_leaf: Option<i32>,

    /// 序号
    #[serde(default)]
    pub ordered: Option<i32>,

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

impl Default for OrgModel {
    fn default() -> Self {
        Self {
            _id: None,
            name: None,
            alias: None,
            province_id: None,
            province: None,
            city_id: None,
            city: None,
            addr: None,
            types: None,
            org_code: None,
            desc: None,
            tel: None,
            parent_id: None,
            is_leaf: None,
            ordered: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
        }
    }
}

impl OrgModel {
    /// 数据库名字
    #[allow(dead_code)]
    pub const DATA_BASE_NAME: &'static str = "YNOS";

    /// 集合名字
    #[allow(dead_code)]
    pub const COLL_NAME: &'static str = "org_list";
}
