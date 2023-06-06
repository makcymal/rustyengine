//! Core engine things

pub mod camera;
pub mod canvas;
pub mod entity;
pub mod event_sys;
pub mod game;
pub mod hype;
pub mod material_traits;
pub mod material;

#[cfg(test)]
mod test;

// re-exports in scope of namespace `engn`
pub use {
    camera::Camera,
    canvas::Canvas,
    entity::{Entity, AsEntity, EntityList, AsGameObject, IdPool},
    game::Game,
    hype::HypePlane,
    event_sys::{AsEvent, AsEventSys},
};
