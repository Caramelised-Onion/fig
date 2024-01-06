// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod model;
use std::sync::Mutex;

use model::Task;
use rusqlite::Connection;
use model::DbModel;

fn main() {
    let conn = Connection::open_in_memory()
        .expect("Could not open a database connection");

    conn.execute(
        "CREATE TABLE tasks (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            time_tracks  TEXT NOT NULL,
            total_time_spent  UInt64 NOT NULL
        )",
        (), // empty list of parameters.
    ).expect("Could not initialise data structure");

    // conn.execute("INSERT INTO tasks (name) VALUES (?1)", ("eat figs",)).expect("Could not enter initial task");
    // conn.execute("INSERT INTO tasks (name) VALUES (?1)", ("floss teeth",)).expect("Could not enter initial task");
    // conn.execute("INSERT INTO tasks (name, time_tracks) VALUES (?1, ?2)", ("figure fig out", "1702905189")).expect("Could not enter initial task");
    
    let initial_tasks = vec![Task::new("Eat figs"), Task::new("Floss teeth"), Task::new("Figure shit out")];

    initial_tasks.iter()
        .for_each(|t| { t.persist(&conn); });
    
    let app_state = AppState {
        db_connection: Mutex::new(conn)
    };

    tauri::Builder::default()
    .manage(app_state)
        .invoke_handler(tauri::generate_handler![create_task, get_all_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn create_task(app_state: tauri::State<'_, AppState>, name: &str) -> Result<String, String> {
    println!("Creating task {}", name);
    // app_state.db_connection.lock().unwrap().execute("INSERT INTO tasks (name) VALUES (?1)", (name,)).unwrap();
    let task = Task::new(name);
    let conn = app_state.db_connection.lock().unwrap();
    task.persist(&conn);
    Ok(name.to_string())
}

#[tauri::command]
async fn get_all_tasks(app_state: tauri::State<'_, AppState>) -> Result<Vec<Task>, String> {
    let conn = app_state.db_connection.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, name, time_tracks, total_time_spent FROM tasks").unwrap();
    let task_names_iter = stmt.query_map([], |row| {
        Ok(Task::from_row(row).unwrap())
    }).unwrap().map(|tr| tr.unwrap());
    Ok(task_names_iter.collect())
}

struct AppState {
    db_connection: Mutex<Connection>
}