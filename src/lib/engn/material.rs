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


/// Entity struct having `id` and properties map
#[derive(Debug)]
pub struct Entity {
    pub(crate) id: Rc<Uuid>,
    pub(crate) props: HashMap<PropKey, PropVal>,
}

impl Entity {
    pub fn new(id: Rc<Uuid>) -> Self {
        Self { id, props: HashMap::new() }
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
    pub(crate) entities: Vec<Rc<RefCell<dyn AsCollided>>>,
}

impl EntityList {
    pub fn new() -> Self {
        Self {
            entities: vec![],
        }
    }
}

impl AsMaterialList for EntityList {
    type Item = Rc<RefCell<dyn AsCollided>>;

    /// Appends new entity that must implement Entity
    fn append(&mut self, item: Self::Item) {
        self.entities.push(item);
    }

    /// Removes entity from the list with the given `Uuid`
    fn remove(&mut self, id: &Rc<Uuid>) {
        self.entities.retain(|entity| Rc::ptr_eq(entity.borrow().id(), id));
    }

    fn get(&self, id: &Rc<Uuid>) -> Option<&Self::Item> {
        if let Some(item) =
            self.entities
                .iter()
                .find(|entity| Rc::ptr_eq(entity.borrow().id(), id))
        {
            Some(&item)
        } else {
            None
        }
    }

    fn exec(&self, f: fn(&Self::Item)) {
        for rc in &self.entities {
            f(rc)
        }
    }

    fn collide(&self, cs: &CoordSys, inc: &Point, dir: &Vector) -> f64 {
        if let Some(dist) = self.entities
            .iter()
            .map(|ent| Float(ent.borrow().collide(cs, inc, dir)))
            .filter(|dist| *dist >= Float(0.0))
            .min()
        {
            dist.into()
        } else {
            -1.0
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
    /// HypePlane constructor takes actual `Entity`, `Point` on plane and normal vector
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

    /// Default instance
    pub fn default(entity: Entity) -> Self {
        Self::new(entity, Point::default(), Vector::new(vec![1.0, 0.0, 0.0])).unwrap()
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

    fn dir(&self) -> &Matrix {
        &self.normal.coord
    }

    fn dir_mut(&mut self) -> &mut Matrix {
        &mut self.normal.coord
    }
}


/// Ellipse in arbitrary dimension space that defined with center point, direction vectors and semiaxes lengths
#[derive(Debug)]
pub struct HypeEllipse {
    pub(crate) entity: Entity,
    pub(crate) center: Point,
    pub(crate) basis: Basis,
    pub(crate) semiaxis: Vec<f64>,
}

impl HypeEllipse {
    /// Constructs new `HypeEllipse`
    pub fn new(entity: Entity, center: Point, basis: Basis, semiaxis: Vec<f64>) -> ReRes<Self> {
        if center.dim() != basis.basis.dim()? {
            return Err(MathErr(DimMismatch { lhs: center.dim(), rhs: basis.basis.dim()? }));
        } else if basis.basis.dim()? != semiaxis.len() {
            return Err(MathErr(DimMismatch { lhs: basis.basis.dim()?, rhs: semiaxis.len() }));
        }
        Ok(Self { entity, center, basis, semiaxis })
    }

    /// Default instance
    pub fn default(entity: Entity) -> Self {
        Self::new(entity, Point::default(), Basis::default(), vec![1.0; 3]).unwrap()
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
        let inc = self.basis.decompose(&inc.df(&self.center).unwrap());
        let dir = self.basis.decompose(dir);
        let (mut a, mut b, mut c) = (0.0, 0.0, -1.0);
        for i in 0..self.center.dim() {
            a += (dir.at(i) / self.semiaxis[i]).powi(2);
            b += 2.0 * dir.at(i) * inc.at(i) / self.semiaxis[i].powi(2);
            c += (inc.at(i) / self.semiaxis[i]).powi(2);
        }
        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            -1.0
        } else if aeq(&d, &0.0) {
            let t = -b / 2.0 / a;
            if t >= 0.0 {
                t
            } else {
                -1.0
            }
        } else {
            [Float((-b + d.sqrt()) / 2.0 / a), Float((-b - d.sqrt()) / 2.0 / a)]
                .iter()
                .filter(|f| *f >= &Float(0.0))
                .min()
                .unwrap_or(&Float(-1.0))
                .into()
        }
    }
}

impl AsGameObject for HypeEllipse {
    fn pos(&self) -> &Point {
        &self.center
    }

    fn pos_mut(&mut self) -> &mut Point {
        &mut self.center
    }

    fn dir(&self) -> &Matrix {
        &self.basis.basis
    }

    fn dir_mut(&mut self) -> &mut Matrix {
        &mut self.basis.basis
    }
}
