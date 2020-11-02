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
    pub fn new(actor_repository: Rc<ActorRepository>) -> Self {
        Self { actor_repository }
    }
}

impl<T> ActorService<T>
where
    T: ActorRepository,
{
    fn actor_repository(&self) -> Rc<ActorRepository> {
        self.actor_repository.clone()
    }
}
