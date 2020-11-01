use crate::domain::models::actor::{actor_id::ActorId, Actor};

pub trait ActorRepository {
    fn find(&self, actor_id: &ActorId) -> Result<Actor, String>;
    fn save(&self, actor: Actor) -> Result<(), String>;
}
