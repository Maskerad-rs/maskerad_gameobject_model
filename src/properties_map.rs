// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::path::{PathBuf, Path};
use properties::transform::Transform;
use properties::mesh::Mesh;

pub struct PropertiesMap {
    transforms: HashMap<PathBuf, Transform>,
    meshes: HashMap<PathBuf, Mesh>,
}

impl PropertiesMap {
    pub fn new() -> Self {
        PropertiesMap {
            transforms: HashMap::new(),
            meshes: HashMap::new(),
        }
    }

    pub fn get_mesh_of(&self, gameobject_id: &Path) -> Option<&Mesh> {
        self.meshes.get(gameobject_id)
    }

    pub fn get_transform_of(&self, gameobject_id: &Path) -> Option<&Transform> {
        self.transforms.get(gameobject_id)
    }

    pub fn add_mesh(&mut self, gameobject_id: &Path, mesh: Mesh) {
        self.meshes.insert(gameobject_id.to_path_buf(), mesh);
    }

    pub fn add_transform(&mut self, gameobject_id: &Path, transform: Transform) {
        self.transforms.insert(gameobject_id.to_path_buf(), transform);
    }
}