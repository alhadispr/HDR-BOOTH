#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Serialize)]
struct AppInfo { name: &'static str, version: &'static str, platform: &'static str }
#[derive(Serialize)]
struct Session { id: i64, name: String, event_type: String, created_at: String, status: String, photo_count: i64 }
#[derive(Serialize)]
struct Capture { id: i64, session_id: i64, file_path: String, created_at: String, print_copies: i64, sync_status: String }

fn db(app: &tauri::AppHandle) -> Result<Connection, String> {
    let mut dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    dir.push("hdr_booth.sqlite3");
    let conn = Connection::open(dir).map_err(|e| e.to_string())?;
    conn.execute_batch(include_str!("../migrations/001_init.sql")).map_err(|e| e.to_string())?;
    Ok(conn)
}

#[tauri::command]
fn app_info() -> AppInfo { AppInfo { name: "HDR BOOTH Professional", version: "1.0.0", platform: "Windows desktop" } }

#[tauri::command]
fn local_data_dir(app: tauri::AppHandle) -> Result<String, String> {
    let p = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&p).map_err(|e| e.to_string())?;
    Ok(p.to_string_lossy().to_string())
}

#[tauri::command]
fn create_session(app: tauri::AppHandle, name: String, event_type: Option<String>) -> Result<i64, String> {
    let conn = db(&app)?;
    conn.execute("INSERT INTO sessions (name,event_type) VALUES (?1,?2)", params![name, event_type.unwrap_or("standard".into())]).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
fn list_sessions(app: tauri::AppHandle) -> Result<Vec<Session>, String> {
    let conn = db(&app)?;
    let mut stmt = conn.prepare("SELECT s.id,s.name,s.event_type,s.created_at,s.status,COUNT(c.id) FROM sessions s LEFT JOIN captures c ON c.session_id=s.id GROUP BY s.id ORDER BY s.id DESC").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |r| Ok(Session { id:r.get(0)?, name:r.get(1)?, event_type:r.get(2)?, created_at:r.get(3)?, status:r.get(4)?, photo_count:r.get(5)? })).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())
}

#[tauri::command]
fn close_session(app: tauri::AppHandle, session_id: i64) -> Result<(), String> {
    let conn = db(&app)?;
    let changed = conn.execute("UPDATE sessions SET status='closed' WHERE id=?1 AND status='active'", params![session_id]).map_err(|e| e.to_string())?;
    if changed == 0 { return Err("Sesi tidak ditemukan atau sudah ditutup".into()); }
    Ok(())
}

#[tauri::command]
fn reopen_session(app: tauri::AppHandle, session_id: i64) -> Result<(), String> {
    let conn = db(&app)?;
    let changed = conn.execute("UPDATE sessions SET status='active' WHERE id=?1", params![session_id]).map_err(|e| e.to_string())?;
    if changed == 0 { return Err("Sesi tidak ditemukan".into()); }
    Ok(())
}

#[tauri::command]
fn add_capture(app: tauri::AppHandle, session_id: i64, file_path: String, print_copies: Option<i64>) -> Result<i64, String> {
    let conn = db(&app)?;
    conn.execute("INSERT INTO captures(session_id,file_path,print_copies) VALUES (?1,?2,?3)", params![session_id,file_path,print_copies.unwrap_or(1)]).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
fn list_captures(app: tauri::AppHandle, session_id: i64) -> Result<Vec<Capture>, String> {
    let conn = db(&app)?;
    let mut stmt = conn.prepare("SELECT id,session_id,file_path,created_at,print_copies,sync_status FROM captures WHERE session_id=?1 ORDER BY id DESC").map_err(|e| e.to_string())?;
    let rows = stmt.query_map(params![session_id], |r| Ok(Capture { id:r.get(0)?, session_id:r.get(1)?, file_path:r.get(2)?, created_at:r.get(3)?, print_copies:r.get(4)?, sync_status:r.get(5)? })).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())
}

#[tauri::command]
fn set_setting(app: tauri::AppHandle, key: String, value: String) -> Result<(), String> {
    let conn = db(&app)?;
    conn.execute("INSERT INTO app_settings(key,value) VALUES (?1,?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value", params![key,value]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_setting(app: tauri::AppHandle, key: String) -> Result<Option<String>, String> {
    let conn = db(&app)?;
    conn.query_row("SELECT value FROM app_settings WHERE key=?1", params![key], |r| r.get(0)).optional().map_err(|e| e.to_string())
}

use rusqlite::OptionalExtension;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![app_info, local_data_dir, create_session, list_sessions, close_session, reopen_session, add_capture, list_captures, set_setting, get_setting])
    .run(tauri::generate_context!())
    .expect("error while running HDR BOOTH");
}
