use super::*;
use model::theme::ThemeModel;

pub struct ThemeService {
    #[allow(dead_code)]
    op: Dao,
}

impl InitCrud for ThemeService {
    fn get_op(&self) -> &Dao {
        &self.op
    }
}

impl ThemeService {
    pub fn new() -> Self {
        Self {
            op: Dao::new(ThemeModel::DATA_BASE_NAME, ThemeModel::COLL_NAME),
        }
    }
}
