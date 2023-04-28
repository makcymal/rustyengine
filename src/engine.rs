use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use {
    crate::{
        linal::{
            matrixify::{
                Vector,
            },
            coord_sys::{
                Point, CoordSys,
            },
        },
    },
    uuid::Uuid,
};


/// Ray in the given coordinate system from the given point with the given direction
pub struct Ray {
    cs: CoordSys,
    initialpt: Point,
    direction: Vector,
}

impl Ray {
    /// The most trivial constructor that can actually exist
    pub fn new(cs: CoordSys, initialpt: Point, direction: Vector) -> Self {
        Self {
            cs,
            initialpt,
            direction,
        }
    }
}


/// Signleton with vector of actual identifiers
pub static mut IDSET: Vec<Id> = vec![];


/// Pseudo-unique identifier based on Uuid v4
#[derive(Debug)]
pub struct Id {
    uuid: Uuid,
}

impl Id {
    /// Generate new identifier that will be added to IDPOOL. Returns ref to elem of vector
    fn generate() -> &'static Self {
        let id = Self {
            uuid: Uuid::new_v4(),
        };
        unsafe {
            IDSET.push(id);
            IDSET.last().unwrap()
        }
    }

    /// Value of current identifier
    pub fn value(&self) -> &Uuid {
        &self.uuid
    }
}


/// Basic engine entity
pub struct Entity<'id> {
    cs: CoordSys,
    id: &'id Id,
    props: HashMap<&'static str, EntityProp>
}

impl<'id> Entity<'id> {
    pub fn new(cs: CoordSys) -> Self {
        Self {
            cs,
            id: Id::generate(),
            props: HashMap::new(),
        }
    }

    pub fn set_prop(&mut self, prop: &str, value: EntityProp) {
        self[prop] = value;
    }

    pub fn get_prop(&self, prop: &str) -> &EntityProp {
        &self[prop]
    }
}

impl<'id> Index<&str> for Entity<'id> {
    type Output = EntityProp;

    fn index(&self, prop: &str) -> &Self::Output {
        self.props.get(prop).expect("Non-existent property")
    }
}

impl<'id> IndexMut<&str> for Entity<'id> {
    fn index_mut(&mut self, prop: &str) -> &mut Self::Output {
        self.props.get_mut(prop).expect("Non-existent property")
    }
}


/// Entity properties
pub enum EntityProp {

}