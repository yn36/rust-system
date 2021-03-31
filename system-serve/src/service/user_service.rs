use super::*;
use model::user::UserModel;

pub struct UserService {
    #[allow(dead_code)]
    op: Dao,
}

impl InitCrud for UserService {
    fn get_op(&self) -> &Dao {
        &self.op
    }
}

impl UserService {
    pub fn new() -> Self {
        Self {
            op: Dao::new(UserModel::DATA_BASE_NAME, UserModel::COLL_NAME),
        }
    }
}
