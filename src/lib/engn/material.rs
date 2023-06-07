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
        grid::Repr,
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


///
#[derive(Debug)]
pub struct Entity {
    pub(crate) id: Rc<Uuid>,
    pub(crate) props: HashMap<PropKey, PropVal>,
}

impl Entity {
    pub fn new(id: &Rc<Uuid>) -> Self {
        Self { id: id.clone(), props: HashMap::new() }
    }
}

impl AsEntity for Entity {
    fn id(&self) -> &Rc<Uuid> {
        &self.id
    }

    fn props(&self) -> &HashMap<PropKey, PropVal> {
        &self.props
    }

    fn props_mut(&mut self) -> &mut HashMap<PropKey, PropVal> {
        &mut self.props
    }
}


///
#[derive(Debug)]
pub struct EntityList {
    pub(crate) entities: Vec<Rc<RefCell<dyn AsEntity>>>,
}

impl AsEntityList for EntityList {
    type Item = Rc<RefCell<dyn AsEntity>>;

    /// Instantiates empty list
    fn new() -> Self {
        Self { entities: vec![] }
    }

    /// Appends new entity that must implement Entity
    fn append(&mut self, item: Self::Item) {
        self.entities.push(item);
    }

    /// Removes entity from the list with the given `Uuid`
    fn remove(&mut self, id: &Rc<Uuid>) {
        self.entities.retain(|entity| Rc::ptr_eq(entity.borrow().id(), id));
    }

    fn iter(&self) -> Box<dyn Iterator<Item=&Self::Item> + '_> {
        Box::new(self.entities.iter())
    }

    fn get(&self, id: &Rc<Uuid>) -> Option<&Self::Item> {
        if let Some(item) =
            self.iter().find(|entity| Rc::ptr_eq(entity.borrow().id(), id))
        {
            Some(&item)
        } else {
            None
        }
    }
}


/// Hype plane defined with some point on it and normal vector
#[derive(Debug)]
pub struct HypePlane {
    pub(crate) entity: Entity,
    pub(crate) initpt: Point,
    pub(crate) normal: Vector,
}

impl HypePlane {
    /// HypePlane constructor takes actual `GameObject`, `Point` on plane and normal vector
    pub fn new(entity: Entity, initpt: Point, mut normal: Vector) -> ReRes<Self> {
        if initpt.dim() != normal.dim() {
            return Err(MathErr(DimMismatch { lhs: initpt.dim(), rhs: normal.dim() }));
        }
        if normal.coord.repr() == Repr::Row {
            normal.coord = normal.coord.transpose();
        }
        Ok(Self {
            entity,
            initpt,
            normal,
        })
    }

    ///
    pub fn default(entity: Entity) -> Self {
        todo!()
    }
}

impl AsEntity for HypePlane {
    fn id(&self) -> &Rc<Uuid> {
        self.entity.id()
    }

    fn props(&self) -> &HashMap<PropKey, PropVal> {
        self.entity.props()
    }

    fn props_mut(&mut self) -> &mut HashMap<PropKey, PropVal> {
        self.entity.props_mut()
    }
}

impl AsCollided for HypePlane {
    fn collide(&self, cs: &CoordSys, inc: &Point, dir: &Vector) -> f64 {
        let denom = cs.scalar_prod(&dir.coord, &self.normal.coord).unwrap();
        if aeq(&denom, &0.0) {
            return -1.0;
        }
        let numer = cs
            .scalar_prod(&self.initpt.df(inc).unwrap().coord, &self.normal.coord)
            .unwrap();
        let dist = numer / denom;
        if dist < 0.0 {
            return -1.0;
        }
        dist
    }
}

impl AsGameObject for HypePlane {
    fn pos(&self) -> &Point {
        &self.initpt
    }

    fn pos_mut(&mut self) -> &mut Point {
        &mut self.initpt
    }

    fn dir(&self) -> &Vector {
        &self.normal
    }

    fn dir_mut(&mut self) -> &mut Vector {
        &mut self.normal
    }
}


///
#[derive(Debug)]
pub struct HypeEllipse {
    pub(crate) entity: Entity,
    pub(crate) cen: Point,
    pub(crate) dir: Vector,
    pub(crate) axis: Vec<f64>,
}

impl HypeEllipse {
    pub fn new(entity: Entity, cen: Point, mut dir: Vector, axis: Vec<f64>) -> ReRes<Self> {
        if cen.dim() != dir.dim() {
            return Err(MathErr(DimMismatch { lhs: cen.dim(), rhs: dir.dim() }));
        } else if dir.dim() != axis.len() {
            return Err(MathErr(DimMismatch { lhs: dir.dim(), rhs: axis.len() }));
        }
        if dir.coord.repr() == Repr::Row {
            dir.coord = dir.coord.transpose()
        }
        Ok(Self { entity, cen, dir, axis })
    }

    ///
    pub fn default(entity: Entity) -> Self {
        todo!()
    }
}

impl AsEntity for HypeEllipse {
    fn id(&self) -> &Rc<Uuid> {
        self.entity.id()
    }

    fn props(&self) -> &HashMap<PropKey, PropVal> {
        self.entity.props()
    }

    fn props_mut(&mut self) -> &mut HashMap<PropKey, PropVal> {
        self.entity.props_mut()
    }
}

impl AsCollided for HypeEllipse {
    fn collide(&self, cs: &CoordSys, inc: &Point, dir: &Vector) -> f64 {
        todo!()
    }
}

impl AsGameObject for HypeEllipse {
    fn pos(&self) -> &Point {
        &self.cen
    }

    fn pos_mut(&mut self) -> &mut Point {
        &mut self.cen
    }

    fn dir(&self) -> &Vector {
        &self.dir
    }

    fn dir_mut(&mut self) -> &mut Vector {
        &mut self.dir
    }
}
