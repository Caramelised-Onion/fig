// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod model;
use std::{env, fs};
use std::{path::PathBuf, sync::Mutex};

use chrono::{DateTime, Utc};
use model::{DbModel, Habit};
use model::Task;
use rusqlite::Connection;

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
            create_habit,
            get_all_habits
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
            name  TEXT NOT NULL,
            time_tracks  TEXT NOT NULL,
            total_time_spent  UInt64 NOT NULL
        );
        CREATE TABLE IF NOT EXISTS habits (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            streak INTEGER,
            time_interval_s INTEGER,
            freq_in_interval INTEGER
        )"
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
    let task = Task::new(name);
    let conn = app_state.db_connection.lock().unwrap();
    task.persist(&conn)
}

#[tauri::command]
async fn get_all_tasks(app_state: tauri::State<'_, AppState>) -> Result<Vec<Task>, String> {
    let conn = app_state.db_connection.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name, time_tracks, total_time_spent FROM tasks")
        .unwrap();
    let task_names_iter = stmt
        .query_map([], |row| Ok(Task::from_row(row).unwrap()))
        .unwrap()
        .map(|tr| tr.unwrap());
    Ok(task_names_iter.collect())
}

#[tauri::command]
async fn add_time_track(
    app_state: tauri::State<'_, AppState>,
    id: usize,
) -> Result<DateTime<Utc>, String> {
    let conn = app_state.db_connection.lock().unwrap();
    let now = Utc::now();
    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id=?1").unwrap();
    let mut task = stmt
        .query_row([id], |row| Ok(Task::from_row(row).unwrap()))
        .unwrap();

    task.add_time_track(now);
    task.update(&conn).unwrap();
    Ok(now)
}

#[tauri::command]
async fn update_task(
    app_state: tauri::State<'_, AppState>,
    updated_task: Task,
) -> Result<(), String> {
    let conn = app_state.db_connection.lock().unwrap();
    match updated_task.update(&conn) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }
}

#[tauri::command]
async fn delete_task(app_state: tauri::State<'_, AppState>, id: usize) -> Result<(), String> {
    let conn = app_state.db_connection.lock().unwrap();
    Task::delete(&conn, id)
}

#[tauri::command]
async fn create_habit(app_state: tauri::State<'_, AppState>, name: &str, time_interval_s: usize, freq_in_interval: usize) -> Result<usize, String> {
    let habit = Habit::new(name, time_interval_s, freq_in_interval);
    let conn = app_state.db_connection.lock().unwrap();
    habit.persist(&conn)
}

// TODO Make macro?? or generics of course
#[tauri::command]
async fn get_all_habits(app_state: tauri::State<'_, AppState>) -> Result<Vec<Habit>, String> {
    let conn = app_state.db_connection.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name, streak, time_interval_s, freq_in_interval FROM habits")
        .unwrap();
    let task_names_iter = stmt
        .query_map([], |row| Ok(Habit::from_row(row).unwrap()))
        .unwrap()
        .map(|tr| tr.unwrap());
    Ok(task_names_iter.collect())
}

struct AppState {
    db_connection: Mutex<Connection>,
}
