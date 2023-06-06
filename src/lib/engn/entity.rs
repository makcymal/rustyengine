


#[derive(Debug)]
pub struct Entity {
    pub(crate) id: Rc<Uuid>,
    pub(crate) props: HashMap<&'static str, Box<dyn Any>>,
}

impl Entity {
    pub fn new(id: &Rc<Uuid>) -> Self {
        Self {
            id: id.clone(),
            props: HashMap::new(),
        }
    }
}

impl AsEntity for Entity {
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




#[derive(Debug)]
pub struct EntityList {
    pub(crate) entities: Vec<Rc<RefCell<dyn Intersected>>>,
}

impl EntityList {
    /// Instantiates empty list
    pub fn new() -> Self {
        Self { entities: vec![] }
    }

    /// Appends new entity that must implement Entity
    pub fn append(&mut self, entity: impl AsGameObject + 'static) {
        self.entities.push(Rc::new(RefCell::new(entity)));
    }

    /// Removes entity from the list with the given `Uuid`
    pub fn remove(&mut self, id: &Rc<Uuid>) {
        self.entities
            .retain(|entity| Rc::ptr_eq(entity.borrow().id(), id));
    }

    /// Returns shared interior mutable ref to entity if exists
    pub fn get(&self, id: &Rc<Uuid>) -> Option<Rc<RefCell<dyn AsGameObject>>> {
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
    pub fn exec(&self, f: fn(Rc<RefCell<dyn AsGameObject>>)) {
        for entity in self.entities.iter() {
            f(entity.clone());
        }
    }
}


pub trait Intersected: AsEntity {

}