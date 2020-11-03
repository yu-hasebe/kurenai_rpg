mod actor;
mod shared;

use crate::application::actor::actor_application_service::ActorApplicationService;
use crate::domain::{
    models::{
        actor::{
            actor_id::ActorId, actor_repository::ActorRepository, direction::Direction,
            speed::Speed, Actor,
        },
        map::{map_id::MapId, map_repository::MapRepository, Map},
        scene::{scene_repository::SceneRepository, Scene},
        shared::{
            canvas::{TILE_HEIGHT, TILE_WIDTH},
            point::Point,
        },
    },
    services::actor_service::ActorService,
};
use kurenai::{
    canvas::CanvasRepository,
    image::{ImageId, ImageRepository},
    key_event::KeyEvent,
    traits::game_service::GameService,
};
use std::rc::Rc;

pub struct RpgGameService<SR, AR, MR>
where
    SR: SceneRepository,
    AR: ActorRepository,
    MR: MapRepository,
{
    actor_application_service: ActorApplicationService<SR, AR, MR>,
}

impl<SR, AR, MR> GameService for RpgGameService<SR, AR, MR>
where
    SR: SceneRepository,
    AR: ActorRepository,
    MR: MapRepository,
{
    fn key_event(&self, key_event: &KeyEvent) {
        self.actor_application_service().key_event(key_event);
    }

    fn update(&self) {
        self.actor_application_service().update();
    }

    fn draw(&self, image_repository: &ImageRepository, canvas_repository: &CanvasRepository) {
        self.actor_application_service()
            .draw(image_repository, canvas_repository);
    }
}

impl<SR, AR, MR> RpgGameService<SR, AR, MR>
where
    SR: SceneRepository,
    AR: ActorRepository,
    MR: MapRepository,
{
    pub fn new() -> Self {
        let scene_repository_rc = Rc::new(SR::new(Scene::new(ActorId(0))));
        let actor_repository_rc = {
            let actor_repository_rc = Rc::new(AR::new());
            let actor = Actor::new(
                ActorId(0),
                ImageId(0),
                Point::new(TILE_WIDTH, TILE_HEIGHT),
                Point::new(0, 0),
                Direction::Down,
                Speed(4),
            );
            actor_repository_rc.save(actor).unwrap();
            actor_repository_rc
        };
        let map_repository_rc = {
            let map_repository_rc = Rc::new(MR::new());
            let map = Map::new(MapId(0), Point::new(960, 960));
            map_repository_rc.save(map).unwrap();
            map_repository_rc
        };
        let actor_service = ActorService::new(
            scene_repository_rc.clone(),
            actor_repository_rc.clone(),
            map_repository_rc,
        );
        let actor_application_service =
            ActorApplicationService::new(actor_service, scene_repository_rc, actor_repository_rc);
        Self {
            actor_application_service,
        }
    }
}

impl<SR, AR, MR> RpgGameService<SR, AR, MR>
where
    SR: SceneRepository,
    AR: ActorRepository,
    MR: MapRepository,
{
    fn actor_application_service(&self) -> &ActorApplicationService<SR, AR, MR> {
        &self.actor_application_service
    }
}
