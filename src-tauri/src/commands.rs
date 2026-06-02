use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use notify::{Watcher, RecursiveMode, Event, EventKind};
use tauri::{Emitter, AppHandle};

use crate::WatcherState;

/// Represents a file system entry for the tree view.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub id: String,
    pub name: String,
    pub entry_type: String, // "file" or "folder"
    pub path: String,
    pub children: Option<Vec<FileEntry>>,
}

/// Supported file extensions
fn is_supported_file(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.ends_with(".md")
        || lower.ends_with(".markdown")
        || lower.ends_with(".html")
        || lower.ends_with(".htm")
}

/// Read a directory and return its contents as a tree structure.
#[tauri::command]
pub fn read_directory(path: String, depth: Option<u32>) -> Result<Vec<FileEntry>, String> {
    let max_depth = depth.unwrap_or(3);
    read_dir_recursive(&PathBuf::from(&path), max_depth, 0)
        .map_err(|e| format!("Failed to read directory: {}", e))
}

fn read_dir_recursive(
    dir: &Path,
    max_depth: u32,
    current_depth: u32,
) -> Result<Vec<FileEntry>, std::io::Error> {
    let mut entries = Vec::new();

    if current_depth >= max_depth {
        return Ok(entries);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden files and directories
        if name.starts_with('.') {
            continue;
        }

        let id = path.to_string_lossy().to_string();

        if path.is_dir() {
            let children = read_dir_recursive(&path, max_depth, current_depth + 1)?;
            entries.push(FileEntry {
                id: id.clone(),
                name,
                entry_type: "folder".to_string(),
                path: id,
                children: Some(children),
            });
        } else if is_supported_file(&name) {
            entries.push(FileEntry {
                id: id.clone(),
                name,
                entry_type: "file".to_string(),
                path: id,
                children: None,
            });
        }
    }

    // Sort: folders first, then files alphabetically
    entries.sort_by(|a, b| match (a.entry_type.as_str(), b.entry_type.as_str()) {
        ("folder", "file") => std::cmp::Ordering::Less,
        ("file", "folder") => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(entries)
}

/// Read the contents of a single file.
#[tauri::command]
pub fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))
}

/// Save content to a file, creating parent directories if needed.
#[tauri::command]
pub fn save_file(path: String, content: String) -> Result<(), String> {
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directories: {}", e))?;
    }
    fs::write(&path, content).map_err(|e| format!("Failed to save file: {}", e))
}

/// Return basic app info for the status bar.
#[tauri::command]
pub fn get_app_info() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "name": "Inkwell MD",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// Start watching a directory for file changes.
/// Emits "file-changed" events to the frontend with the changed file path.
#[tauri::command]
pub fn watch_directory(
    app: AppHandle,
    state: tauri::State<WatcherState>,
    path: String,
) -> Result<(), String> {
    // Stop any existing watcher for this path
    {
        let mut watchers = state.watchers.lock().unwrap();
        watchers.remove(&path);
    }

    let watch_path = path.clone();
    let app_handle = app.clone();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        match res {
            Ok(event) => {
                let dominated = matches!(
                    event.kind,
                    EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
                );
                if dominated {
                    for changed_path in event.paths {
                        let name = changed_path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();

                        // Skip hidden files
                        if name.starts_with('.') {
                            continue;
                        }

                        // Only emit for supported file types
                        if is_supported_file(&name) {
                            let path_str = changed_path.to_string_lossy().to_string();
                            let event_kind = format!("{:?}", event.kind);
                            let _ = app_handle.emit(
                                "file-changed",
                                serde_json::json!({
                                    "path": path_str,
                                    "name": name,
                                    "kind": event_kind,
                                    "watchRoot": watch_path,
                                }),
                            );
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("File watcher error: {}", e);
            }
        }
    })
    .map_err(|e| format!("Failed to create watcher: {}", e))?;

    watcher
        .watch(Path::new(&path), RecursiveMode::Recursive)
        .map_err(|e| format!("Failed to watch directory: {}", e))?;

    // Store the watcher so it stays alive
    let mut watchers = state.watchers.lock().unwrap();
    watchers.insert(path, watcher);

    Ok(())
}

/// Stop watching a directory.
#[tauri::command]
pub fn unwatch_directory(
    state: tauri::State<WatcherState>,
    path: String,
) -> Result<(), String> {
    let mut watchers = state.watchers.lock().unwrap();
    watchers.remove(&path);
    Ok(())
}
