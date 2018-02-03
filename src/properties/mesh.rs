// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use gltf::Gltf;

//TODO: I don't know how to use Gltf data, WIP.
#[derive(Debug)]
pub struct Mesh {
    data: Gltf,
}

impl Mesh {
    pub fn new(data: Gltf) -> Self {
        Mesh {
            data
        }
    }

    pub fn data(&self) -> &Gltf {
        &self.data
    }
}