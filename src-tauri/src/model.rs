
use std::{collections::HashSet, ops::Not};
use chrono::{prelude::*, Duration};
use serde::{Deserialize, Serialize};
use crate::entities::{IntervalModel, TaskModel};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub time_tracks: Vec<Interval>,
    pub total_time_spent: usize,
}

impl Task {
    pub fn is_ongoing(&self) -> bool {
        self.time_tracks.last().map_or_else(|| false, |tt| tt.is_open())
    }
    
    pub fn total_time_spent(&self) -> usize {
        self.total_time_spent
    }
    
    fn calculate_total_time_spent(&self) -> usize {
        self.time_tracks.iter()
            .map(|time_track| time_track.time_spent())
            .map(|time_spent| time_spent.num_seconds() as usize)
            .sum()
    }
}

pub struct Interval {
    pub id: usize,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

impl Interval {
    fn new(self) {

    }
}


// See https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
// impl From<IntervalModel> for Interval {
//     fn from(w: IntervalModel) -> Interval {
//         w.0
//     }  
// }

// // -> tags/catogeries -> DAG
// // -> time tracking
// // -> pomdoro timer/stats
// // -> habits tracking (with streaks) (link tasks/todo with habits)
// // -> integration with calendar (certain tasks/todo can show up in your calnedar
// // -> calendar: plan vs actual execution (time and whatever intervals maths would be interesting)

// // TODO repeated task?
// // optional fields like due date

// pub struct IntervalModel {
//     pub id: usize,
//     pub start_time: DateTime<Utc>,
//     pub end_time: Option<DateTime<Utc>>,
//     pub task_id: usize,
// }

// impl IntervalModel {
//     pub fn new(task_id: usize) -> Self {
//         Self {
//             id: 0,
//             start_time: Utc::now(),
//             end_time: None,
//             task_id,
//         }
//     }

//     pub fn is_open(&self) -> bool {
//         self.end_time.is_some()
//     }

//     pub fn time_spent(&self) -> Duration {
//         self.end_time
//             .unwrap_or_else(|| Utc::now())
//             .signed_duration_since(self.start_time)
//     }
// }

// impl DbModel for IntervalModel {
//     fn persist(&self, conn: &Connection) -> Result<usize, String> {
//         let insert_result = match self.end_time {
//             Some(end_time) => conn.execute(
//                 "INSERT INTO intervals (start_time, end_time, task_id) VALUES (?1, ?2, ?3)",
//                 (
//                     self.start_time.timestamp(),
//                     end_time.timestamp(),
//                     self.task_id,
//                 ),
//             ),
//             None => conn.execute(
//                 "INSERT INTO intervals (start_time, task_id) VALUES (?1, ?2)",
//                 (self.start_time.timestamp(), self.task_id),
//             ),
//         };
//         match insert_result {
//             Ok(_) => Ok(conn.last_insert_rowid() as usize),
//             Err(err) => Err(err.to_string()),
//         }
//     }

//     fn update(&self, conn: &Connection) -> Result<(), String> {
//         todo!()
//     }

//     fn delete(conn: &Connection, id: usize) -> Result<(), String> {
//         todo!()
//     }

//     fn from_row(row: &Row) -> Result<Self, String>
//     where
//         Self: Sized,
//     {
//         let start_time = DateTime::<Utc>::from_timestamp(row.get(1).unwrap(), 0).unwrap();
//         let end_time: Option<DateTime<Utc>> = row
//             .get(2)
//             .ok()
//             .map(|ts| DateTime::from_timestamp(ts, 0))
//             .flatten();

//         Ok(Self {
//             id: row.get(0).unwrap(),
//             start_time,
//             end_time,
//             task_id: row.get(3).unwrap(),
//         })
//     }

//     fn get_all(conn: &Connection) -> Vec<Self>
//     where
//         Self: Sized,
//     {
//         todo!()
//     }
// }

// pub struct TaskModel {
//     pub id: usize,
//     pub name: String,
// }

// impl TaskModel {
//     pub fn new(name: &str) -> Self {
//         Self {
//             id: 0,
//             name: name.to_string(),
//         }
//     }

//     pub fn is_ongoing(&self) -> bool {
//         self.time_tracks.last().map_or_else(|| false, |tt| tt.is_open())
//     }

//     pub fn total_time_spent(&self) -> usize {
//         self.total_time_spent
//     }

//     fn calculate_total_time_spent(&self) -> usize {
//         self.time_tracks.iter()
//             .map(|time_track| time_track.time_spent())
//             .map(|time_spent| time_spent.num_seconds() as usize)
//             .sum()
//     }
// }

// impl DbModel for TaskModel {
//     // TODO PhatomData for this??
//     fn persist(&self, conn: &Connection) -> Result<usize, String> {
//         let time_tracks_repr: String = serde_json::to_string(&self.time_tracks).unwrap();

//         println!("time_tracks_repr: {:?}", time_tracks_repr);
//         conn.execute(
//             "INSERT INTO tasks (name, time_tracks, total_time_spent) VALUES (?1, ?2, ?3)",
//             (self.name.clone(), time_tracks_repr, self.total_time_spent),
//         )
//         .unwrap();
//         Ok(conn.last_insert_rowid() as usize)
//     }

//     fn update(&self, conn: &Connection) -> Result<(), String> {
//         // TODO: only update the fields that actually need updating
//         let time_tracks_repr: String = serde_json::to_string(&self.time_tracks).unwrap();
//         conn.execute(
//             "UPDATE tasks SET name=?1, time_tracks=?2, total_time_spent=?3 WHERE id=?4",
//             (
//                 self.name.clone(),
//                 time_tracks_repr,
//                 self.total_time_spent,
//                 self.id,
//             ),
//         )
//         .unwrap();
//         Ok(())
//     }

//     fn delete(conn: &Connection, id: usize) -> Result<(), String> {
//         conn.execute("DELETE FROM tasks WHERE id=?1", [id]).unwrap();
//         Ok(())
//     }

//     fn from_row(row: &Row) -> Result<Self, String> {
//         let serialized_time_tracks: String = row.get(2).unwrap();
//         let time_tracks: Vec<DateTime<Utc>> =
//             serde_json::from_str(&serialized_time_tracks).unwrap();

//         Ok(TaskModel {
//             id: row.get(0).unwrap(),
//             name: row.get(1).unwrap(),
//             time_tracks: time_tracks,
//             total_time_spent: row.get(3).unwrap(),
//             // total_time_spent: 0
//         })
//     }

//     fn get_all(conn: &Connection) -> Vec<Self> {
//         todo!()
//     }
// }

// struct Category {
//     name: String,
//     id: u32,
//     parents: HashSet<u32>,
//     children: HashSet<u32>,
// }

// pub struct Habit {
//     pub id: usize,
//     pub name: String,
//     pub streak: usize,
//     pub time_interval_s: usize,
//     pub freq_in_interval: usize,
// }

// impl DbModel for Habit {
//     fn persist(&self, conn: &Connection) -> Result<usize, String> {
//         conn.execute(
//             "INSERT INTO habits (name, streak, time_interval_s, freq_in_interval) VALUES (?1, ?2, ?3, ?4)",
//             (self.name.clone(), self.streak, self.time_interval_s, self.freq_in_interval),
//         )
//         .unwrap();
//         Ok(conn.last_insert_rowid() as usize)
//     }

//     fn update(&self, conn: &Connection) -> Result<(), String> {
//         todo!()
//     }

//     fn delete(conn: &Connection, id: usize) -> Result<(), String> {
//         conn.execute("DELETE FROM habits WHERE id=?1", [id])
//             .unwrap();
//         Ok(())
//     }

//     fn from_row(row: &Row) -> Result<Self, String>
//     where
//         Self: Sized,
//     {
//         Ok(Habit {
//             id: row.get(0).unwrap(),
//             name: row.get(1).unwrap(),
//             streak: row.get(2).unwrap(),
//             time_interval_s: row.get(3).unwrap(),
//             freq_in_interval: row.get(4).unwrap(),
//         })
//     }

//     fn get_all(conn: &Connection) -> Vec<Self> {
//         todo!()
//     }
// }

// impl Habit {
//     pub fn new(name: &str, time_interval_s: usize, freq_in_interval: usize) -> Self {
//         Self {
//             id: 0,
//             name: name.to_string(),
//             streak: 0,
//             time_interval_s,
//             freq_in_interval,
//         }
//     }

//     fn increment(&mut self) {
//         todo!()
//     }
// }
