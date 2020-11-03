use crate::domain::models::actor::actor_repository::ActorRepository;
use derive_new::new;
use std::rc::Rc;

#[derive(Clone, Debug, new)]
pub struct ActorService<T>
where
    T: ActorRepository,
{
    actor_repository_rc: Rc<T>,
}

impl<T> ActorService<T>
where
    T: ActorRepository,
{
    fn actor_repository_rc(&self) -> Rc<T> {
        self.actor_repository_rc.clone()
    }
}
