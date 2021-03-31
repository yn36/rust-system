//！ 省份模型

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProvinceModel {
    #[serde(default)]
    pub _id: Option<String>,

    /// 省份名称
    #[serde(default)]
    pub province_name: Option<String>,

    /// 省份编码
    #[serde(default)]
    pub province_code: Option<String>,

    /// 省份别名
    #[serde(default)]
    pub short_name: Option<String>,

    /// 省份经度
    #[serde(default)]
    pub lng: Option<String>,

    /// 省份纬度
    #[serde(default)]
    pub lat: Option<String>,

    /// 省份排序
    #[serde(default)]
    pub sort: Option<String>,

    /// 描述
    #[serde(default)]
    pub desc: Option<String>,

    /// 创建时间
    #[serde(default)]
    pub create_time: Option<String>,

    /// 修改时间
    #[serde(default)]
    pub update_time: Option<String>,
}

impl Default for ProvinceModel {
    fn default() -> Self {
        Self {
            _id: None,
            province_name: None,
            province_code: None,
            short_name: None,
            lng: None,
            lat: None,
            sort: None,
            desc: None,
            create_time: None,
            update_time: None,
        }
    }
}

impl ProvinceModel {
    /// 数据库名字
    #[allow(dead_code)]
    pub const DATA_BASE_NAME: &'static str = "position";

    /// 集合名字
    #[allow(dead_code)]
    pub const COLL_NAME: &'static str = "provinces";
}
