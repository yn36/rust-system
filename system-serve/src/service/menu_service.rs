use super::*;
use model::menu::{AuthMenuModel, MenuModel};

pub struct MenuService {
    #[allow(dead_code)]
    op: Dao,
}

impl InitCrud for MenuService {
    fn get_op(&self) -> &Dao {
        &self.op
    }
}

impl MenuService {
    pub fn new() -> Self {
        Self {
            op: Dao::new(MenuModel::DATA_BASE_NAME, MenuModel::COLL_NAME),
        }
    }
}

pub struct AuthMenuService {
    #[allow(dead_code)]
    op: Dao,
}

impl InitCrud for AuthMenuService {
    fn get_op(&self) -> &Dao {
        &self.op
    }
}

impl AuthMenuService {
    pub fn new() -> Self {
        Self {
            op: Dao::new(AuthMenuModel::DATA_BASE_NAME, AuthMenuModel::COLL_NAME),
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
