use rusqlite::{Connection, Row};
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

pub trait DbModel {
    /// Returns primary key of the inserted entity
    fn persist(&self, conn: &Connection) -> Result<usize, String>;
    fn update(&self, conn: &Connection) -> Result<(), String>;
    fn delete(conn: &Connection, id: usize) -> Result<(), String>;
    fn from_row(row: &Row) -> Result<Self, String>
    where
        Self: Sized;
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: usize,
    pub name: String,
    // categories: HashSet<Category>,
    // habit: Option<Habit>,
    // due_date: Option<DateTime<Utc>>,
    pub time_tracks: Vec<DateTime<Utc>>,
    pub total_time_spent: usize,
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
    pub fn new(name: &str) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            time_tracks: Vec::new(),
            total_time_spent: 0,
        }
    }

    pub fn total_time_spent(&self) -> usize {
        self.total_time_spent
    }
    pub fn add_time_track(&mut self, timestamp: DateTime<Utc>) {
        self.time_tracks.push(timestamp);
        if self.time_tracks.len() % 2 == 0 {
            self.total_time_spent = self.calculate_total_time_spent();
        }
    }

    fn calculate_total_time_spent(&self) -> usize {
        19
    }
}

impl DbModel for Task {
    // TODO PhatomData for this??
    fn persist(&self, conn: &Connection) -> Result<usize, String> {
        let time_tracks_repr: String = serde_json::to_string(&self.time_tracks).unwrap();

        println!("time_tracks_repr: {:?}", time_tracks_repr);
        conn.execute(
            "INSERT INTO tasks (name, time_tracks, total_time_spent) VALUES (?1, ?2, ?3)",
            (self.name.clone(), time_tracks_repr, self.total_time_spent),
        )
        .unwrap();
        Ok(conn.last_insert_rowid() as usize)
    }

    fn update(&self, conn: &Connection) -> Result<(), String> {
        // TODO: only update the fields that actually need updating
        let time_tracks_repr: String = serde_json::to_string(&self.time_tracks).unwrap();
        conn.execute(
            "UPDATE tasks  SET name=?1, time_tracks=?2, total_time_spent=?3 WHERE id=?4",
            (
                self.name.clone(),
                time_tracks_repr,
                self.total_time_spent,
                self.id,
            ),
        )
        .unwrap();
        Ok(())
    }

    fn delete(conn: &Connection, id: usize) -> Result<(), String> {
        conn.execute("DELETE FROM tasks WHERE id=?1", [id]).unwrap();
        Ok(())
    }

    fn from_row(row: &Row) -> Result<Self, String> {
        let serialized_time_tracks: String = row.get(2).unwrap();
        let time_tracks: Vec<DateTime<Utc>> =
            serde_json::from_str(&serialized_time_tracks).unwrap();

        Ok(Task {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            time_tracks: time_tracks,
            total_time_spent: row.get(3).unwrap(),
            // total_time_spent: 0
        })
    }
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
