//！ 职务模型

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobModel {
    #[serde(default)]
    pub _id: Option<String>,

    /// 职务名称
    #[serde(default)]
    pub job_name: Option<String>,

    /// 职务编码
    #[serde(default)]
    pub job_code: Option<String>,

    /// 职务描述
    #[serde(default)]
    pub job_desc: Option<String>,

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

impl Default for JobModel {
    fn default() -> Self {
        Self {
            _id: None,
            job_name: None,
            job_code: None,
            job_desc: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
        }
    }
}

impl JobModel {
    /// 数据库名字
    #[allow(dead_code)]
    pub const DATA_BASE_NAME: &'static str = "YNOS";

    /// 集合名字
    #[allow(dead_code)]
    pub const COLL_NAME: &'static str = "job_list";
}
