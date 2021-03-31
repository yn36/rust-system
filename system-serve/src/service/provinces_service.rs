use super::*;
use model::provinces::ProvinceModel;

pub struct ProvinceService {
    #[allow(dead_code)]
    op: Dao,
}

impl InitCrud for ProvinceService {
    fn get_op(&self) -> &Dao {
        &self.op
    }
}

impl ProvinceService {
    pub fn new() -> Self {
        Self {
            op: Dao::new(ProvinceModel::DATA_BASE_NAME, ProvinceModel::COLL_NAME),
        }
    }
}
