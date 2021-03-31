use super::*;
use model::job::JobModel;

pub struct JobService {
    #[allow(dead_code)]
    op: Dao,
}

impl InitCrud for JobService {
    fn get_op(&self) -> &Dao {
        &self.op
    }
}

impl JobService {
    pub fn new() -> Self {
        Self {
            op: Dao::new(JobModel::DATA_BASE_NAME, JobModel::COLL_NAME),
        }
    }
}
