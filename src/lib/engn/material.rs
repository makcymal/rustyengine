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
    pub(crate) entities: Vec<Rc<RefCell<dyn AsEntity>>>,
}

impl EntityList {
    pub fn new() -> Self {
        Self {
            entities: vec![],
        }
    }
}

impl AsEntityList for EntityList {
    type Item = Rc<RefCell<dyn AsEntity>>;

    /// Appends new entity that must implement Entity
    fn append(&mut self, item: Self::Item) {
        self.entities.push(item);
    }

    /// Removes entity from the list with the given `Uuid`
    fn remove(&mut self, id: &Rc<Uuid>) {
        self.entities.retain(|entity| Rc::ptr_eq(entity.borrow().id(), id));
    }

    fn exec(&self, f: fn(&Self::Item)) {
        for rc in &self.entities {
            f(rc)
        }
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
}


/// Hype plane defined with some point on it and normal vector
#[derive(Debug)]
pub struct HypePlane {
    pub(crate) initpt: Point,
    pub(crate) normal: Vector,
}

impl HypePlane {
    /// HypePlane constructor takes actual `Entity`, `Point` on plane and normal vector
    pub fn new(initpt: Point, normal: Vector) -> Self {
        Self {
            initpt,
            normal,
        }
    }
}

impl AsCollided for HypePlane {
    fn collide(&self, inc: &Point, dir: &Vector) -> f32 {
        let denom = dir.scalar_prod(&self.normal);
        if aeq(denom, 0.0) {
            return -1.0;
        }
        let numer = &self.initpt.df(inc).scalar_prod(&self.normal);
        let dist = numer / denom;
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


/// Ellipse in arbitrary dimension space that defined with center point, direction vectors and semiaxes lengths
#[derive(Debug)]
pub struct HypeEllipse {
    pub(crate) center: Point,
    pub(crate) basis: Basis,
    pub(crate) semiaxis: [f32; 3],
}

impl HypeEllipse {
    /// Constructs new `HypeEllipse`
    pub fn new(center: Point, basis: Basis, semiaxis: [f32; 3]) -> Self {
        Self { center, basis, semiaxis }
    }
}

impl AsCollided for HypeEllipse {
    fn collide(&self, inc: &Point, dir: &Vector) -> f32 {
        let inc = self.basis.decompose(&inc.df(&self.center));
        let dir = self.basis.decompose(dir);
        let (mut a, mut b, mut c) = (0.0, 0.0, -1.0);
        for i in 0..3 {
            a += (dir[i] / self.semiaxis[i]).powi(2);
            b += 2.0 * dir[i] * inc[i] / self.semiaxis[i].powi(2);
            c += (inc[i] / self.semiaxis[i]).powi(2);
        }
        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            -1.0
        } else if aeq(d, 0.0) {
            -b / 2.0 / a
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

    fn dir(&self) -> &Vector {
        &self.basis[0]
    }

    fn dir_mut(&mut self) -> &mut Vector {
        &mut self.basis[0]
    }

    fn planar_rotate(&mut self, from: usize, to: usize, angle: f32) {
        let rot = Matrix::rotation(from, to, angle);
        for i in 0..3 {
            self.basis[i] = &rot * &self.basis[i];
        }
    }
}
