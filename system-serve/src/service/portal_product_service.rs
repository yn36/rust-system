use super::*;
use model::portal_product::PortalProductModel;

pub struct PortalProductService {
    #[allow(dead_code)]
    pub op: Dao,
}

impl InitCrud for PortalProductService {
    fn get_op(&self) -> &Dao {
        &self.op
    }
}

impl PortalProductService {
    pub fn new() -> Self {
        Self {
            op: Dao::new(PortalProductModel::DATA_BASE_NAME, PortalProductModel::COLL_NAME),
        }
    }
}
