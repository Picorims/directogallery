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

use tauri::api::dir::DiskEntry;

/// State of the gallery exploration by the user
#[derive(Debug)]
pub struct Gallery {
    exploration_stack: Vec<Vec<DiskEntry>>
}

impl Gallery {
    /// Creates an empty gallery
    pub fn new() -> Self {
        Gallery { exploration_stack: vec![] }
    }

    /// recreate the gallery based on the new provided root content.
    pub fn set_root(&mut self, root: Vec<DiskEntry>) {
        self.exploration_stack = vec![];
        self.exploration_stack.push(root);
    }
}
