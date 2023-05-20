// This file is part of Directogallery

// Directogallery is an application to browse a directory of images
// from the file system in a more convenient way.
// Copyright (C) 2023  Charly Schmidt alias Picorims<picorims.contact@gmail.com>

// Directogallery is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Directogallery is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Directogallery.  If not, see <https://www.gnu.org/licenses/>.


// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;

use std::{sync::Mutex, path::PathBuf};

use models::gallery::Gallery;
use tauri::{AppHandle, State};

#[derive(Debug)]
enum GalleryStateValue {
    Nil,
    Gallery(Gallery)
}

/// mutable gallery
struct GalleryState(Mutex<GalleryStateValue>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
/// Recursively caches the content of the provided directory, considered the root
/// of the gallery.
fn cache_root(path: &str, state: State<GalleryState>, app_handle: AppHandle) -> Result<(), String> {
    use tauri::api::dir;

    if dir::is_dir(path).map_err(|e| e.to_string())? {
        // take the mutex before changing its value
        let mut gallery = state.0.lock().unwrap();
        *gallery = GalleryStateValue::Gallery(Gallery::new(PathBuf::from(path)).map_err(|e| e.to_string())?);
        let gallery = gallery;
        println!("{:?}", gallery);
    }

    Ok(())
}

#[tauri::command]
/// Return the JSON structure of the current directory (not recursive)
fn get_current_dir_data(state: tauri::State<GalleryState>) -> Option<serde_json::Value> {
    todo!();
    let gallery = state.0.lock().unwrap();
    // gallery.current_dir_as_json()
}

fn main() {
    tauri::Builder::default()
        .manage(GalleryState(Mutex::new(GalleryStateValue::Nil)))
        .invoke_handler(tauri::generate_handler![
            cache_root,
            get_current_dir_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
