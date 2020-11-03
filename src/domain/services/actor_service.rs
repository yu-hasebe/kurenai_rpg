use crate::domain::models::{
    actor::actor_repository::ActorRepository, map::map_repository::MapRepository,
    scene::scene_repository::SceneRepository,
};
use derive_new::new;
use std::rc::Rc;

#[derive(Clone, Debug, new)]
pub struct ActorService<SR, AR, MR>
where
    SR: SceneRepository,
    AR: ActorRepository,
    MR: MapRepository,
{
    scene_repository_rc: Rc<SR>,
    actor_repository_rc: Rc<AR>,
    map_repository_rc: Rc<MR>,
}

impl<SR, AR, MR> ActorService<SR, AR, MR>
where
    SR: SceneRepository,
    AR: ActorRepository,
    MR: MapRepository,
{
    fn scene_repository_rc(&self) -> Rc<SR> {
        self.scene_repository_rc.clone()
    }

    fn actor_repository_rc(&self) -> Rc<AR> {
        self.actor_repository_rc.clone()
    }

    fn map_repository_rc(&self) -> Rc<MR> {
        self.map_repository_rc.clone()
    }
}
