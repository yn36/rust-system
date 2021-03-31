use super::*;
use model::city::CityModel;

pub struct CityService {
    #[allow(dead_code)]
    op: Dao,
}

impl InitCrud for CityService {
    fn get_op(&self) -> &Dao {
        &self.op
    }
}

impl CityService {
    pub fn new() -> Self {
        Self {
            op: Dao::new(CityModel::DATA_BASE_NAME, CityModel::COLL_NAME),
        }
    }
}
