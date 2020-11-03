use crate::domain::models::map::{map_id::MapId, map_repository::MapRepository, Map};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct InMemoryMapRepository {
    store: Rc<RefCell<HashMap<MapId, Map>>>,
}

impl MapRepository for InMemoryMapRepository {
    fn new() -> Self {
        Self {
            store: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    fn find(&self, map_id: &MapId) -> Result<Map, String> {
        match self.store.borrow().get(map_id) {
            Some(r) => Ok(r.clone()),
            None => Err("Not found in MapRepository".to_string()),
        }
    }

    fn save(&self, map: Map) -> Result<(), String> {
        self.store.borrow_mut().insert(*map.id(), map);
        Ok(())
    }
}
