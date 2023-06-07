//! Core engine things

pub mod camera;
pub mod canvas;
pub mod console;
pub mod event;
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
    event::{
        AsEvent, ConsoleEvent, AsEventSys, EventQueue,
    },
    material_traits::{
        AsEntity,
        AsCollided,
        AsGameObject,
        AsMaterialList,
        PropKey, PropVal,
    },
    material::{
        IdPool,
        Entity,
        EntityList,
        HypePlane,
        HypeEllipse,
    },
};
