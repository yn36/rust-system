use super::*;
use model::role::{RoleModel, UserRoleModel};

/// 角色表
pub struct RoleService {
    #[allow(dead_code)]
    op: Dao,
}

/// 用户角色表多对多
pub struct UserRoleService {
    #[allow(dead_code)]
    op: Dao,
}

/// 初始化角色service
impl InitCrud for RoleService {
    fn get_op(&self) -> &Dao {
        &self.op
    }
}

/// 初始化用户角色service
impl InitCrud for UserRoleService {
    fn get_op(&self) -> &Dao {
        &self.op
    }
}

impl RoleService {
    pub fn new() -> Self {
        Self {
            op: Dao::new(RoleModel::DATA_BASE_NAME, RoleModel::COLL_NAME),
        }
    }
}

impl UserRoleService {
    pub fn new() -> Self {
        Self {
            op: Dao::new(UserRoleModel::DATA_BASE_NAME, UserRoleModel::COLL_NAME),
        }
    }

    /// 保存多条数据
    pub async fn save_role(
        &self,
        datas: impl IntoIterator<Item = Document>,
    ) -> Result<mongodb::results::InsertManyResult, BusinessError> {
        self.op.save_many(datas).await
    }
}
