// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod entities;
pub mod model;

use std::{env, fs};
use std::{path::PathBuf, sync::Mutex};

use chrono::{DateTime, Utc};
use entities::{IntervalEntity, TaskEntity};
use model::Task;
use rusqlite::Connection;

use crate::entities::Entity;

fn main() {
    let app_state = startup().expect("error while initialising app state");
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            create_task,
            get_all_tasks,
            add_time_track,
            delete_task,
            update_task,
            // create_habit,
            // get_all_habits
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn startup() -> Result<AppState, std::io::Error> {
    let conn = get_database_connection()?;
    create_table(&conn)?;
    Ok(AppState {
        db_connection: Mutex::new(conn),
    })
}

fn create_table(conn: &Connection) -> Result<(), std::io::Error> {
    let res = conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS tasks (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS habits (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            streak INTEGER,
            time_interval_s INTEGER,
            freq_in_interval INTEGER
        );
        CREATE TABLE IF NOT EXISTS intervals (
            id    INTEGER PRIMARY KEY,
            start_time  INTEGER NOT NULL,
            end_time INTEGER,
            task_id  INTEGER,
            FOREIGN KEY(task_id) REFERENCES tasks(id)
        );",
    );
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            format!(
                "Could not initialise data structure, original error: {}",
                err.to_string()
            ),
        )),
    }
}

fn get_database_connection() -> Result<Connection, std::io::Error> {
    let home = env::var("HOME").expect("please set $HOME env var");
    let mut path = PathBuf::from(&home);
    path.push(".fig");
    fs::create_dir_all(&path)?;
    path.push("fig.db3");

    match Connection::open(&path) {
        Ok(conn) => Ok(conn),
        Err(err) => Err(std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            format!(
                "Failed to open a db connection, original error: {}",
                err.to_string()
            ),
        )),
    }
}

#[tauri::command]
async fn create_task(app_state: tauri::State<'_, AppState>, name: &str) -> Result<usize, String> {
    println!("Creating task {}", name);
    let task = TaskEntity::new(name);
    let conn = app_state.db_connection.lock().unwrap();
    task.persist(&conn)
}

#[tauri::command]
async fn get_all_tasks(app_state: tauri::State<'_, AppState>) -> Result<Vec<Task>, String> {
    let conn = app_state.db_connection.lock().unwrap();
    // let mut stmt = conn
    //     .prepare("SELECT tasks.id, name FROM tasks LEFT JOIN intervals ON intervals.task_id = tasks.id")
    //     .unwrap();
    let mut select_tasks = conn
        .prepare("SELECT * FROM tasks")
        .unwrap();
    let tasks_iter = select_tasks
        .query_map([], |row| Ok(TaskEntity::from_row(row).unwrap()))
        .unwrap()
        .map(|tr| tr.unwrap());
    
    let mut res: Vec<Task> = vec![];
    for task_entity in tasks_iter {
        let mut select_intervals_for_task = conn
            .prepare("SELECT * FROM intervals WHERE task_id = ?1")
            .unwrap();
        let intervals_iter: Vec<IntervalEntity> = select_intervals_for_task
            .query_map([task_entity.id], |row| Ok(IntervalEntity::from_row(row).unwrap()))
            .unwrap()
            .map(|ir| ir.unwrap()).collect();
        
        let task = Task::from_entities(&task_entity, intervals_iter);
        // create
        res.push(task);
    }
    

    Ok(res)
}

#[tauri::command]
async fn add_time_track(
    app_state: tauri::State<'_, AppState>,
    task_id: usize,
) -> Result<Task, String> {
    let conn = app_state.db_connection.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM intervals WHERE task_id=?1  ORDER BY start_time DESC LIMIT 1;")
        .unwrap();
    let latest_open_interval = stmt
        .query_row([task_id], |row| Ok(IntervalEntity::from_row(row).unwrap()))
        .ok()
        .filter(|interval| interval.is_open());

    match latest_open_interval {
        Some(mut interval) => {
            interval.end_time = Some(Utc::now());
            interval.update(&conn).unwrap();
        },
        None => {
            let new_interval = IntervalEntity::new(task_id);
            new_interval.persist(&conn).unwrap();
        }
    }

    let task_entity = conn
        .prepare("SELECT * FROM tasks WHERE id=?1")
        .unwrap()
        .query_row([task_id], |row| Ok(TaskEntity::from_row(row).unwrap()))
        .unwrap();

    let mut select_intervals_for_task = conn
        .prepare("SELECT * FROM intervals WHERE task_id = ?1")
        .unwrap();
    let interval_entities: Vec<IntervalEntity> = select_intervals_for_task
        .query_map([task_entity.id], |row| Ok(IntervalEntity::from_row(row).unwrap()))
        .unwrap()
        .map(|ir| ir.unwrap()).collect();

    let task = Task::from_entities(&task_entity, interval_entities);

    Ok(task)
}

#[tauri::command]
async fn update_task(
    app_state: tauri::State<'_, AppState>,
    updated_task: Task,
) -> Result<(), String> {
    let conn = app_state.db_connection.lock().unwrap();
    let task_entity: TaskEntity = updated_task.into();
    match task_entity.update(&conn) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn delete_task(app_state: tauri::State<'_, AppState>, id: usize) -> Result<(), String> {
    let conn = app_state.db_connection.lock().unwrap();
    TaskEntity::delete(&conn, id)
}

// #[tauri::command]
// async fn create_habit(
//     app_state: tauri::State<'_, AppState>,
//     name: &str,
//     time_interval_s: usize,
//     freq_in_interval: usize,
// ) -> Result<usize, String> {
//     let habit = Habit::new(name, time_interval_s, freq_in_interval);
//     let conn = app_state.db_connection.lock().unwrap();
//     habit.persist(&conn)
// }

// // TODO Make macro?? or generics of course
// #[tauri::command]
// async fn get_all_habits(app_state: tauri::State<'_, AppState>) -> Result<Vec<Habit>, String> {
//     let conn = app_state.db_connection.lock().unwrap();
//     let mut stmt = conn
//         .prepare("SELECT id, name, streak, time_interval_s, freq_in_interval FROM habits")
//         .unwrap();
//     let task_names_iter = stmt
//         .query_map([], |row| Ok(Habit::from_row(row).unwrap()))
//         .unwrap()
//         .map(|tr| tr.unwrap());
//     Ok(task_names_iter.collect())
// }

struct AppState {
    db_connection: Mutex<Connection>,
}
