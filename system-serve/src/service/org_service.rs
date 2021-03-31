use super::*;
use model::org::OrgModel;

pub struct OrgService {
    #[allow(dead_code)]
    pub op: Dao,
}

impl InitCrud for OrgService {
    fn get_op(&self) -> &Dao {
        &self.op
    }
}

impl OrgService {
    pub fn new() -> Self {
        Self {
            op: Dao::new(OrgModel::DATA_BASE_NAME, OrgModel::COLL_NAME),
        }
    }
}
