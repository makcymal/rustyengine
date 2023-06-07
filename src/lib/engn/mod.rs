//! Core engine things

pub mod camera;
pub mod canvas;
pub mod console;
pub mod event_sys;
pub mod game;
pub mod material_traits;
pub mod material;

#[cfg(test)]
mod test;

// re-exports in scope of namespace `engn`
pub use {
    camera::Camera,
    canvas::Canvas,
    game::Game,
    event_sys::{AsEvent, AsEventSys},
    material_traits::{
        AsEntity,
        AsEntityList,
        AsCollided,
        AsCollidedList,
        AsGameObject,
        PropKey, PropVal,
    },
    material::{IdPool, Entity, EntityList, HypePlane},
};
