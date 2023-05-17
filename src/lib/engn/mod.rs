//! Core engine things

mod ray;
mod entity;
mod game;

// re-exports in scope of namespace `engine`
pub use {
    ray::Ray,
    entity::{
        IdPool, EntityCore, Entity, Prop, EntityList, GameObject, Camera,
    },
    game::Game,
};
