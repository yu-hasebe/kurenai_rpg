use crate::domain::models::{
    actor::actor_repository::ActorRepository, scene::scene_repository::SceneRepository,
};
use derive_new::new;
use std::rc::Rc;

#[derive(Clone, Debug, new)]
pub struct ActorService<SR, AR>
where
    SR: SceneRepository,
    AR: ActorRepository,
{
    scene_repository_rc: Rc<SR>,
    actor_repository_rc: Rc<AR>,
}

impl<SR, AR> ActorService<SR, AR>
where
    SR: SceneRepository,
    AR: ActorRepository,
{
    fn scene_repository_rc(&self) -> Rc<SR> {
        self.scene_repository_rc.clone()
    }

    fn actor_repository_rc(&self) -> Rc<AR> {
        self.actor_repository_rc.clone()
    }
}
