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

use std::sync::Mutex;

use models::gallery::Gallery;

/// mutable gallery
struct GalleryState(Mutex<Gallery>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
/// Recursively caches the content of the provided directory, considered the root
/// of the gallery.
fn cache_root(path: &str, state: tauri::State<GalleryState>) -> Result<(), String> {
    use tauri::api::dir;

    if dir::is_dir(path).map_err(|e| e.to_string())? {
        let dir_content = dir::read_dir(path, true).map_err(|e| e.to_string())?;

        // take the mutex before changing its value
        let mut gallery = state.0.lock().unwrap();
        gallery.set_root(dir_content);
        println!("{:?}", gallery);
    }

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(GalleryState(Mutex::new(Gallery::new())))
        .invoke_handler(tauri::generate_handler![cache_root])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
