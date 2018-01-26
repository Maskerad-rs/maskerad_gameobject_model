// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate cgmath;

pub mod properties_map;
pub mod gameobject;
pub mod properties;
/*
    We can have:
    - static geometry
    - dynamic geometry

    gameobject = actor, dynamic element



    WORLD EDITOR
    ----------------
    - level creation and management
    - 3D view of the level
    - navigation in the level
    - object selection/inspection/editing
    - group objects in layers
    - object placement and alignment aids (snap to grid/terrain...)
    - special object types (light -> no model, particle emitter, sounds, regions, splines, nav mesh...)
    - save/load levels
    - integrated asset management tools

    GAMEPLAY FOUNDATION SYSTEMS
    ------------------------------
    - runtime game object model
    - level management and streaming
    - real time object updating
    - message and event handling
    - scripting
    - game flow management

    runtime game object model : spawn/destroy objects, links to low level subsystems, update of game objects, create new game objects easily,
    gameobject queries, gameobject reference, state machine support, save load objects...

    We want a PROPERTY-CENTRIC game object model.

    PROPERTY-CENTRIC DESIGN
    ------------------------
    Instead of thinking with objects like this :
    Object1:
        - pos [x, y, z]
        - health u32
    Object2:
        - ...
        -...
    ...

    We think like this :
    Pos:
        - object1 [x, y, z]
        - object2 [x, y, z]
   health:
        - object1 u32
        - object2 u32
   ...

    We have a GameObjectsAttributes structure, holding hashmaps of <gameobject ID, componentValue> like this:
    GameObjectComponents {
        positions: HashMap<PathBuf, Position>
        healths: HashMap<PathBuf, u32>
        ...
    }

    impl GameObjectsAttribute {
        pub fn get_pos(&self, id: &Path).
    }

    In this object, we can find all gameobjects' attributes.
    For behavior ? either :
     - Scripts, my own lisp-like shitty language if possible.
     - "Property class" used by the gameobject class: components.

     if scripts:
     GameObject {
        id: PathBuf,
        script_id: SomethingToLinkTheScriptToTheGameObject,
     }

     Or a mix of the two: some critical property classes (position...) and a facility to implement custom properties via scripts.
*/