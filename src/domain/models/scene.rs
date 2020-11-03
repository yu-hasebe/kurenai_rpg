pub mod scene_repository;

use crate::domain::models::actor::actor_id::ActorId;
use derive_new::new;

#[derive(Clone, Debug, new)]
pub struct Scene {
    actor_id: ActorId,
}

impl Scene {
    pub fn actor_id(&self) -> &ActorId {
        &self.actor_id
    }
}
