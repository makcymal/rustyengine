use {
    super::*,
    crate::{
        errs::{
            GameErr::{self, *},
            GridErr::{self, *},
            MathErr::{self, *},
            ReErr::{self, *},
            ReRes,
        },
        math::*,
    },
    std::{
        any::{Any, TypeId},
        cell::RefCell,
        collections::hash_map::{Entry, HashMap},
        ops::{Index, IndexMut},
        rc::Rc,
    },
    uuid::Uuid,
};

/// Matrix of `Uuid` (standard v4) allocated in heap
#[derive(Debug, Default, Clone, PartialEq)]
pub struct IdPool {
    pub(crate) ids: Vec<Rc<Uuid>>,
}

impl IdPool {
    /// Empty constructor
    pub fn new() -> Self {
        Self { ids: vec![] }
    }

    /// Method scoped in `engine` namespace, generates `Uuid` of v4
    pub fn generate(&mut self) -> Rc<Uuid> {
        self.ids.push(Rc::new(Uuid::new_v4()));
        Rc::clone(self.ids.last().unwrap())
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }
}

impl Index<usize> for IdPool {
    type Output = Uuid;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ids[index]
    }
}

/// Main trait for any entity instance requires returning UUID and map of properties
pub trait Entity {
    /// UUID of entity
    fn id(&self) -> &Rc<Uuid>;

    /// Ref to map of properties
    fn props(&self) -> &HashMap<&'static str, Box<dyn Any>>;

    /// Mutable ref to map of properties
    fn props_mut(&mut self) -> &mut HashMap<&'static str, Box<dyn Any>>;

    /// Inserts new pair `key`: `val` into `props` field or replaces already existing
    fn set_prop(&mut self, key: &'static str, val: Box<dyn Any>) {
        match self.props_mut().entry(key) {
            Entry::Occupied(o) => *o.into_mut() = val,
            Entry::Vacant(v) => {
                let _ = v.insert(val);
            }
        };
    }

    /// Returns `ReRes` with ref to requested `Box<dyn Any>` instance or meaningful error if key doesn't exist
    fn get_prop(&self, key: &'static str) -> ReRes<&Box<dyn Any>> {
        if let Some(prop) = self.props().get(key) {
            Ok(prop)
        } else {
            Err(GameErr(NotInitializedProp))
        }
    }

    /// Performs deleting value by the given `Prop` key
    fn del_prop(&mut self, key: &'static str) {
        self.props_mut().remove(key);
    }
}

impl std::fmt::Debug for dyn Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UUID {:?}", self.id())
    }
}

impl Index<&'static str> for dyn Entity {
    type Output = Box<dyn Any>;

    fn index(&self, key: &'static str) -> &Self::Output {
        &self.props()[key]
    }
}

#[derive(Debug)]
pub struct Core {
    pub(crate) id: Rc<Uuid>,
    pub(crate) props: HashMap<&'static str, Box<dyn Any>>,
}

impl Core {
    pub fn new(id: &Rc<Uuid>) -> Self {
        Self {
            id: id.clone(),
            props: HashMap::new(),
        }
    }
}

impl Entity for Core {
    fn id(&self) -> &Rc<Uuid> {
        &self.id
    }

    fn props(&self) -> &HashMap<&'static str, Box<dyn Any>> {
        &self.props
    }

    fn props_mut(&mut self) -> &mut HashMap<&'static str, Box<dyn Any>> {
        &mut self.props
    }
}

pub trait GameObject: Entity {
    fn pos(&self) -> &Point;

    fn pos_mut(&mut self) -> &mut Point;

    fn dir(&self) -> &Vector;

    fn dir_mut(&mut self) -> &mut Vector;

    fn intersect(&self, cs: &CoordSys, ray: &Ray) -> f64;

    fn mv(&mut self, vec: &Vector) -> ReRes<()> {
        self.pos_mut().mv_assign(vec)
    }

    fn df(&self, pt: &Point) -> ReRes<Vector> {
        self.pos().df(pt)
    }

    fn rotate_3d(&mut self, x: f64, y: f64, z: f64) -> ReRes<()> {
        self.dir_mut().coord = self.dir().coord.mul(&Matrix::teit_bryan_rotation(x, y, z));
        self.dir().coord.ag_failed()?;
        Ok(())
    }

    fn planar_rotate(&mut self, from: usize, to: usize, angle: f64) -> ReRes<()> {
        self.dir_mut().coord = self.dir().coord.mul(&Matrix::rotation(from, to, angle, 3));
        self.dir().coord.ag_failed()?;
        Ok(())
    }

    /// Dimension of space where `GameObject` lays
    fn dim(&self) -> usize {
        self.pos().dim()
    }
}

impl std::fmt::Debug for dyn GameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UUID {:?}\n Position {:?}\n Direction {:?}",
            self.id(),
            self.pos(),
            self.dir()
        )
    }
}

#[derive(Debug)]
pub struct EntityList {
    pub(crate) entities: Vec<Rc<RefCell<dyn GameObject>>>,
}

impl EntityList {
    /// Instantiates empty list
    pub fn new() -> Self {
        Self { entities: vec![] }
    }

    /// Appends new entity that must implement Entity
    pub fn append(&mut self, entity: impl GameObject + 'static) {
        self.entities.push(Rc::new(RefCell::new(entity)));
    }

    /// Removes entity from the list with the given `Uuid`
    pub fn remove(&mut self, id: &Rc<Uuid>) {
        self.entities
            .retain(|entity| Rc::ptr_eq(entity.borrow().id(), id));
    }

    /// Returns shared interior mutable ref to entity if exists
    pub fn get(&self, id: &Rc<Uuid>) -> Option<Rc<RefCell<dyn GameObject>>> {
        if let Some(rc) = self
            .entities
            .iter()
            .find(|entity| Rc::ptr_eq(entity.borrow().id(), id))
        {
            Some(rc.clone())
        } else {
            None
        }
    }

    /// Permorms closure that may be immutable due to interior mutability
    pub fn exec(&self, f: fn(Rc<RefCell<dyn GameObject>>)) {
        for entity in self.entities.iter() {
            f(entity.clone());
        }
    }
}
