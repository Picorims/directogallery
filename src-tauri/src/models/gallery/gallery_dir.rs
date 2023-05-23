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

use serde_json::json;
use tauri::api::dir::DiskEntry;

// https://developer.mozilla.org/fr/docs/Web/HTML/Element/img
// https://stackoverflow.com/a/42764117 - reference to a static array of references to static strings
const ALLOWED_IMG_EXTENSIONS: &'static [&'static str] = &["apng", "avif", "gif", "jpg", "jpeg", "jpe", "jif", "jfif", "png", "svg", "webp"];

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
    parent: Option<Weak<Mutex<GalleryDir>>>,
    directories: Mutex<Vec<Arc<Mutex<GalleryDir>>>>,
    files: Vec<DiskEntry>,
    name: String,
    path: PathBuf
}

impl GalleryDir {
    pub fn new(path: PathBuf, parent: Option<Weak<Mutex<GalleryDir>>>) -> Result<Self, CreationError> {
        // if the directory name is invalid, it is likely that the directory itself
        // is not fitting as well, as per the file_name function specification.
        // If it can't be converted to a string, it is more likely that it contains
        // unknown characters which is less of a problem.
        let name_os_str = path.file_name().ok_or(CreationError)?;
        Ok(GalleryDir {
            parent: parent,
            directories: Mutex::new(vec![]),
            files: vec![],
            name: String::from(name_os_str.to_str().unwrap_or("unknown")),
            path: path,
        })
    }

    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_path(&self) -> PathBuf { self.path.clone() }
    pub fn get_files_json(&self) -> serde_json::Value {
        json!(self.files)
    }
    pub fn get_dirs_json(&self) -> serde_json::Value {
        let directories = self.directories.lock().unwrap();
        let names: Vec<_> = directories.iter().map(|d| {
            d.lock().unwrap().get_name()
        }).collect();
        json!(names)
    }

    pub fn get_dir_by_name(&self, name: String) -> Option<Weak<Mutex<GalleryDir>>> {
        let directories = self.directories.lock().unwrap();
        let dir = directories.iter()
            .find(|d| {d.lock().unwrap().get_name() == name});
        match dir {
            None => None,
            Some(d) => Some(Arc::downgrade(d))
        }
    }

    pub fn get_parent(&self) -> Option<Weak<Mutex<GalleryDir>>> {
        match &self.parent {
            Some(p) => Some(Weak::clone(p)),
            None => None
        }
    }

    pub fn add_file(&mut self, file: DiskEntry) {
        self.files.push(file);
    }

    pub fn add_dir(&mut self, dir: Arc<Mutex<GalleryDir>>) {
        self.directories.lock().unwrap().push(dir);
    }

    /// fill the provided GalleryDir with file and directories of the content vector,
    /// and creates its children GalleryDir for directories.
    pub fn fill(&mut self, content: Vec<DiskEntry>, parent: Weak<Mutex<GalleryDir>>) -> Result<(), CreationError> {
        for entry in content {
            if entry.children.is_none() {
                // file
                let extension = entry.path.extension();
                if extension.is_some() {
                    let extension_str = extension.unwrap();

                    // only keep image files
                    ALLOWED_IMG_EXTENSIONS.iter()
                        .any(|str| {extension_str.eq_ignore_ascii_case(str)})
                        .then(|| {self.add_file(entry);});
                }
                
            } else {
                // directory
                let child_dir = Arc::new(Mutex::new(GalleryDir::new(entry.path, Some(Weak::clone(&parent)))?));
                let children_vec = entry.children.ok_or(CreationError)?;
                let mut child_dir_mut = child_dir.lock().unwrap();
                child_dir_mut.fill(children_vec, Arc::downgrade(&child_dir))?;
                self.add_dir(Arc::clone(&child_dir));
            }
        }
        Ok(())
    }
}

impl fmt::Display for GalleryDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{{parent: {:?}, directories: {}, files: {}, name: {}, path: {:?})}}",
            self.parent, self.directories.lock().unwrap().len(), self.files.len(), self.name, self.path
        )
    }
}