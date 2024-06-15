use serde::{Deserialize, Serialize};

use crate::model::Task;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OngoingTasksUpdated {
    pub updated_tasks: Vec<Task>,
}

impl OngoingTasksUpdated {
    pub fn new(updated_tasks: Vec<Task>) -> Self {
        Self{ updated_tasks }
    }
}
