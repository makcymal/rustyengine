mod ray;
mod entity;
mod game;

// re-exports in scope of namespace `engine`
pub use {
    ray::Ray,
    entity::{
        IdSet, EntityCore, Entity, Entitify, EntityList, GameObject, GameCamera,
    },
    game::Game,
};