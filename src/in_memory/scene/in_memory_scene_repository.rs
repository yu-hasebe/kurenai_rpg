use crate::domain::models::scene::{scene_repository::SceneRepository, Scene};
use std::{cell::RefCell, rc::Rc};

pub struct InMemorySceneRepository {
    store: Rc<RefCell<Scene>>,
}

impl SceneRepository for InMemorySceneRepository {
    fn new(scene: Scene) -> Self {
        Self {
            store: Rc::new(RefCell::new(scene)),
        }
    }

    fn find(&self) -> Result<Scene, String> {
        Ok(self.store.borrow().clone())
    }

    fn save(&self, scene: Scene) -> Result<(), String> {
        *self.store.borrow_mut() = scene;
        Ok(())
    }
}
