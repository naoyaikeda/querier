use rusqlite::Connection;
use std::env;
use dotenvy::dotenv;

use std::sync::Mutex;

pub struct DbState {
    pub conn: Mutex<Connection>,
}

use tauri::{AppHandle, Manager};

pub fn establish_connection(app: &AppHandle) -> Result<Connection, Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = if let Ok(url) = env::var("DATABASE_URL") {
        url
    } else {
        let app_data_dir = app.path().app_data_dir()?;
        if !app_data_dir.exists() {
            std::fs::create_dir_all(&app_data_dir)?;
        }
        app_data_dir.join("sqlite.db").to_string_lossy().to_string()
    };

    let conn = Connection::open(database_url)?;
    println!("Successfully connected to SQLite database!");
    Ok(conn)
}
