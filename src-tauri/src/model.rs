use std::collections::HashSet;
use std::time::Duration;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

// -> tags/catogeries -> DAG
// -> time tracking
// -> pomdoro timer/stats
// -> habits tracking (with streaks) (link tasks/todo with habits)
// -> integration with calendar (certain tasks/todo can show up in your calnedar
// -> calendar: plan vs actual execution (time and whatever intervals maths would be interesting)


// TODO repeated task?
// optional fields like due date

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: usize,
    pub name: String,
    // categories: HashSet<Category>,
    // habit: Option<Habit>,
    // due_date: Option<DateTime<Utc>>,
    // time_tracks: Vec<DateTime<Utc>>, 
}

impl Task {
    // fn complete(&mut self) {
    //     self.habit.as_mut().map(|h| h.increment());
    // }
    // fn is_in_progress(&self) -> bool {
    //     self.time_tracks.len() % 2 == 1
    // }
    // fn change_session_state(&mut self) {
    //     self.time_tracks.push(Utc::now());
    // }
    // fn total_time_spent(&self) -> u32 {
    //     todo!()
    // }
}


struct Category {
    name: String,
    id: u32,
    parents: HashSet<u32>,
    children: HashSet<u32>,
}

struct Habit {
    name: String,
    streak: usize,
    time_interval: Duration,
    freq_in_interval: usize,
}


impl Habit {
    fn increment(&mut self) {
        todo!()
    }
}