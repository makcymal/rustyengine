//! Core engine things

mod camera;
mod canvas;
mod entity;
mod game;
mod geometrical;

#[cfg(test)]
mod test;

// re-exports in scope of namespace `engine`
pub use {
    camera::Camera,
    canvas::Canvas,
    entity::{
        Entity, IdPool, EntityCore, GameObject,
    },
    geometrical::{
        Intersected, EntityList, HypePlane,
    },
    game::Game,
};
