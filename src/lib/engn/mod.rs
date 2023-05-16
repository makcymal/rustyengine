mod ray;
mod entity;
mod game;

// re-exports in scope of namespace `engine`
pub use {
    ray::Ray,
    entity::{
        IdPool, EntityCore, Entity, EntityList, GameObject, GameCamera,
    },
    game::Game,
};
