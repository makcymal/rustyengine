//! `rustyengine` core!
//! Has the following features:
//! 1. Console drawing with ASCI characters that can be specified
//! 2. Traits for materials that can be treaten as game entites, eg planes, ellipses, empty entities,
//! that for example can keeps game state variables. Define yourself entities by implemeting provided traits.
//! 3. Traits for materials stores that can be processed during event handling
//! 4. Traits for events and event sustems as well as simple event queue
//! 5. Game object defined with the given implementation of provided traits and parameters from
//! `Conf` that stands for configuration read from `.toml` file


pub mod camera;
pub mod canvas;
pub mod console;
pub mod event;
pub mod event_traits;
pub mod game;
pub mod material_traits;
pub mod material;

#[cfg(test)]
mod test;

pub use {
    camera::Camera,
    canvas::Canvas,
    game::Game,
    event::{
        MovementEvent, MovementEventSys
    },
    event_traits::{
        AsEvent, AsEventSys
    },
    material_traits::{
        AsEntity,
        AsCollided,
        AsGameObject,
        AsEntityList,
        AsScene,
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
