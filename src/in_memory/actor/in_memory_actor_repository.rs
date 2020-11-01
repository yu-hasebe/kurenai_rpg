use crate::domain::models::actor::{actor_id::ActorId, actor_repository::ActorRepository, Actor};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct InMemoryActorRepository {
    store: Rc<RefCell<HashMap<ActorId, Actor>>>,
}

impl ActorRepository for InMemoryActorRepository {
    fn find(&self, actor_id: &ActorId) -> Result<Actor, String> {
        match self.store.borrow().get(actor_id) {
            Some(r) => Ok(r.clone()),
            None => Err("Not found in ActorRepository".to_string()),
        }
    }

    fn save(&self, actor: Actor) -> Result<(), String> {
        self.store.borrow_mut().insert(*actor.actor_id(), actor);
        Ok(())
    }
}

impl InMemoryActorRepository {
    pub fn new() -> Self {
        Self {
            store: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}
