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

use std::{path::PathBuf};

use tauri::api::dir::DiskEntry;

use gallery_dir::GalleryDir;

use self::gallery_dir::CreationError;

/// State of the gallery exploration by the user
#[derive(Debug)]
pub struct Gallery {
    root: Option<GalleryDir>
}

impl Gallery {
    /// Creates an empty gallery
    pub fn new() -> Self {
        Gallery { root: None }
    }

    /// recreate the gallery based on the new provided root content
    /// (recursively explored directory).
    pub fn process_root(&mut self, root_path: PathBuf, root_content: Vec<DiskEntry>) -> Result<(), CreationError> {
        let mut root_dir = GalleryDir::new(root_path)?;
        Self::fill_gallery_dir(&mut root_dir, root_content)?;
        self.root = Some(root_dir);
        Ok(())
    }

    fn fill_gallery_dir(dir: &mut GalleryDir, content: Vec<DiskEntry>) -> Result<(), CreationError> {
        for entry in content {
            if entry.children.is_none() {
                // file
                dir.add_file(entry);
            } else {
                // directory
                let mut child_dir = GalleryDir::new(entry.path)?;
                let children_vec = entry.children.ok_or(
                    CreationError
                )?;
                Self::fill_gallery_dir(&mut child_dir, children_vec)?;
                dir.add_dir(child_dir);
            }
        }
        Ok(())
    }
}
