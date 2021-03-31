use super::*;
use model::button::ButtonModel;
pub struct ButtonService {
    #[allow(dead_code)]
    op: Dao,
}

impl InitCrud for ButtonService {
    fn get_op(&self) -> &Dao {
        &self.op
    }
}

impl ButtonService {
    pub fn new() -> Self {
        Self {
            op: Dao::new(ButtonModel::DATA_BASE_NAME, ButtonModel::COLL_NAME),
        }
    }
}
