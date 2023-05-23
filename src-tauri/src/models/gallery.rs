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

mod gallery_dir;

use std::{path::PathBuf, sync::{Mutex, Weak, Arc}, error, fmt};

use serde_json::json;

use gallery_dir::GalleryDir;
use tauri::api::dir;

/// Error thrown when the GalleryDir encounters a problem
#[derive(Debug)]
pub struct JSONError;
impl error::Error for JSONError {}
impl fmt::Display for JSONError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The Gallery could not be converted to JSON.")
    }
}

/// Error thrown when the GalleryDir cannot navigate to another directory
#[derive(Debug)]
pub struct NavError;
impl error::Error for NavError {}
impl fmt::Display for NavError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The Gallery could not navigate to the requested directory.")
    }
}


/// State of the gallery exploration by the user
#[derive(Debug)]
pub struct Gallery {
    root: Arc<Mutex<GalleryDir>>,
    current_dir: Weak<Mutex<GalleryDir>>
}

impl Gallery {
    /// Creates an empty gallery
    pub fn new(path: PathBuf) -> Result<Self, Box<dyn error::Error>> {
        let dir_content = dir::read_dir(&path, true)?;
        let root = Arc::new(Mutex::new(GalleryDir::new(path, None)?));

        let gallery = Gallery {
            root: Arc::clone(&root),
            current_dir: Arc::downgrade(&root) // empty
        };
        
        // gallery.root automatically deref to the mutex
        gallery.root.lock().unwrap().fill(dir_content, Arc::downgrade(&root))?;
        Ok(gallery)
    }

    pub fn current_dir_as_json(&self) -> Result<serde_json::Value, JSONError> {
        let current_dir_arc = self.current_dir.upgrade().ok_or(JSONError)?;
        let current_dir = current_dir_arc.lock().unwrap();

        Ok(json!({
            "name": current_dir.get_name(),
            "path": current_dir.get_path(),
            "files": current_dir.get_files_json(),
            "directories": current_dir.get_dirs_json()
        }))
    }

    /// modify the current dir to a child dir of the current dir selected by its name
    pub fn explore_child_dir(&mut self, name: String) -> Result<(), NavError> {
        let arc_pointer = self.current_dir.upgrade().ok_or(NavError)?;
        let new_dir = arc_pointer.lock().unwrap().get_dir_by_name(name);
        self.current_dir = new_dir.ok_or(NavError)?;
        Ok(())
    }

    /// modify the current dir to the parent dir if it exists.
    pub fn explore_parent_dir(&mut self) -> Result<(), NavError> {
        let arc_pointer = self.current_dir.upgrade().ok_or(NavError)?;
        let new_dir = arc_pointer.lock().unwrap().get_parent();
        self.current_dir = new_dir.ok_or(NavError)?;
        Ok(())
    }
}
