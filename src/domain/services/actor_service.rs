use crate::domain::models::actor::actor_repository::ActorRepository;
use derive_new::new;
use std::rc::Rc;

#[derive(Clone, Debug, new)]
pub struct ActorService<T>
where
    T: ActorRepository,
{
    actor_repository: Rc<T>,
}

impl<T> ActorService<T>
where
    T: ActorRepository,
{
    fn actor_repository(&self) -> Rc<T> {
        self.actor_repository.clone()
    }
}
