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

use std::{path::PathBuf, error, fmt, sync::{Mutex, Arc, Weak}};

use tauri::api::dir::DiskEntry;

/// Error thrown when the GalleryDir encounters a problem
#[derive(Debug)]
pub struct CreationError;
impl error::Error for CreationError {}
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The GalleryDir could not be created correctly.")
    }
}

/// represent a node of the directory tree of the gallery
#[derive(Debug)]
pub struct GalleryDir {
    // see https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
    // and https://doc.rust-lang.org/book/ch16-03-shared-state.html
    directories: Mutex<Vec<Arc<Mutex<GalleryDir>>>>,
    parent: Weak<Mutex<GalleryDir>>,
    files: Vec<DiskEntry>,
    name: String,
    path: PathBuf
}

impl GalleryDir {
    pub fn new(path: PathBuf) -> Result<Self, CreationError> {
        // if the directory name is invalid, it is likely that the directory itself
        // is not fitting as well, as per the file_name function specification.
        // If it can't be converted to a string, it is more likely that it contains
        // unknown characters which is less of a problem.
        let name_os_str = path.file_name().ok_or(CreationError)?;
        Ok(GalleryDir {
            directories: Mutex::new(vec![]),
            parent: Weak::new(),
            files: vec![],
            name: String::from(name_os_str.to_str().unwrap_or("unknown")),
            path: path,
        })
    }

    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_path(&self) -> PathBuf { self.path.clone() }

    pub fn add_file(&mut self, file: DiskEntry) {
        self.files.push(file);
    }

    pub fn add_dir(&mut self, dir: Arc<Mutex<GalleryDir>>) {
        self.directories.lock().unwrap().push(dir);
    }

    /// fill the provided GalleryDir with file and directories of the content vector,
    /// and creates its children GalleryDir for directories.
    pub fn fill(&mut self, content: Vec<DiskEntry>) -> Result<(), CreationError> {
        for entry in content {
            if entry.children.is_none() {
                // file
                self.add_file(entry);
            } else {
                // directory
                let mut child_dir = Arc::new(Mutex::new(GalleryDir::new(entry.path)?));
                let children_vec = entry.children.ok_or(
                    CreationError
                )?;
                child_dir.lock().unwrap().fill(children_vec)?;
                self.add_dir(Arc::clone(&child_dir));
                self.parent = Arc::downgrade(&child_dir);
            }
        }
        Ok(())
    }
}