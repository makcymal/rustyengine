//! Core engine things

mod camera;
mod canvas;
mod entity;
mod game;
mod hype;

#[cfg(test)]
mod test;

// re-exports in scope of namespace `engn`
pub use {
    camera::{Camera, Ray},
    canvas::Canvas,
    entity::{Core, Entity, EntityList, GameObject, IdPool},
    game::Game,
    hype::HypePlane,
};
