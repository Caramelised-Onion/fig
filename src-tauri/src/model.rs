use chrono::{prelude::*, Duration};
use serde::{Deserialize, Serialize};
use crate::entities::{IntervalEntity, TaskEntity};

#[derive(Serialize, Deserialize)]
pub struct Interval {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

impl Interval {
    pub fn new() -> Self {
        Self{
            start_time: Utc::now(),
            end_time: None
        }
    }

    pub fn from_entity(entity: &IntervalEntity) -> Self {
        Self {
            start_time: entity.start_time,
            end_time: entity.end_time
        }
    }
    
    fn time_spent(&self) -> Duration {
        match self.end_time {
            Some(end_time) => end_time - self.start_time,
            None => Utc::now() - self.start_time,
        }
    }

    fn is_open(&self) -> bool {
        self.end_time.is_none()
    }
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub intervals: Vec<Interval>,
    pub total_time_spent: i64, // stored in seconds
    pub is_open: bool,
}

impl Into<TaskEntity> for Task {
    fn into(self) -> TaskEntity {
        TaskEntity {
            id: self.id,
            name: self.name,
        }
    }
}

impl Task {
    pub fn from_entities(entity: &TaskEntity, interval_entities: Vec<IntervalEntity>) -> Self {
        let intervals = interval_entities.iter()
            .map(|ie| Interval::from_entity(ie)).collect();
        let mut task = Self {
            id: entity.id,
            name: entity.name.clone(),
            intervals,
            total_time_spent: 0,
            is_open: false,
        };
        task.total_time_spent = task.total_time_spent().num_seconds();
        task.is_open = task.is_ongoing();
        task
    }

    pub fn is_ongoing(&self) -> bool {
        self.intervals.last().map_or_else(|| false, |tt| tt.is_open())
    }
    
    pub fn total_time_spent(&self) -> Duration {
        self.intervals.iter()
            .map(|interval| interval.time_spent())
            .sum()
    }
}
