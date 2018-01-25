// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cgmath::prelude::*;
use cgmath::Vector3;

#[derive(Debug)]
pub struct Transform {
    position: Vector3<f32>,
    rotation: Vector3<f32>, //TODO: Probably quaternion
    scale: Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Transform {
    pub fn new(position: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>) -> Self {
        Transform {
            position,
            rotation,
            scale,
        }
    }

    pub fn position(&self) -> &Vector3<f32> {
        &self.position
    }

    pub fn rotation(&self) -> &Vector3<f32> {
        &self.rotation
    }

    pub fn scale(&self) -> &Vector3<f32> {
        &self.scale
    }

    pub fn set_position(&mut self, new_pos: (f32, f32, f32)) {
        self.position = Vector3::from(new_pos);
    }

    pub fn set_rotation(&mut self, new_rot: (f32, f32, f32)) {
        self.rotation = Vector3::from(new_rot)
    }

    pub fn set_scale(&mut self, new_scale: (f32, f32, f32)) {
        self.scale = Vector3::from(new_scale)
    }
}