use crate::domain::models::{
    actor::actor_repository::ActorRepository,
    map::map_repository::MapRepository,
    scene::scene_repository::SceneRepository,
    shared::{
        canvas::{
            CANVAS_HEIGHT_IN_TILE, CANVAS_WIDTH_IN_TILE, HALF_CANVAS_HEIGHT_IN_TILE,
            HALF_CANVAS_WIDTH_IN_TILE, TILE_HEIGHT, TILE_WIDTH,
        },
        point::{Dot, Point},
    },
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
    pub fn actor_at_on_canvas(&self) -> Point<Dot> {
        let actor = self
            .actor_repository_rc()
            .find(self.scene_repository_rc().find().unwrap().actor_id())
            .unwrap();
        let map = self.map_repository_rc().find(actor.map_id()).unwrap();
        Point::new(
            Self::actor_at_x_on_canvas(*actor.at().x(), *map.size().x()),
            Self::actor_at_y_on_canvas(*actor.at().y(), *map.size().y()),
        )
    }
}

impl<SR, AR, MR> ActorService<SR, AR, MR>
where
    SR: SceneRepository,
    AR: ActorRepository,
    MR: MapRepository,
{
    fn actor_at_x_on_canvas(actor_at_x: i64, map_width: i64) -> i64 {
        if actor_at_x < HALF_CANVAS_WIDTH_IN_TILE * TILE_WIDTH {
            actor_at_x
        } else if actor_at_x < map_width - (HALF_CANVAS_WIDTH_IN_TILE + 1) * TILE_WIDTH {
            HALF_CANVAS_WIDTH_IN_TILE * TILE_WIDTH
        } else {
            CANVAS_WIDTH_IN_TILE * TILE_WIDTH + actor_at_x - map_width
        }
    }

    fn actor_at_y_on_canvas(actor_at_y: i64, map_height: i64) -> i64 {
        if actor_at_y < HALF_CANVAS_HEIGHT_IN_TILE * TILE_HEIGHT {
            actor_at_y
        } else if actor_at_y < map_height - (HALF_CANVAS_HEIGHT_IN_TILE + 1) * TILE_HEIGHT {
            HALF_CANVAS_HEIGHT_IN_TILE * TILE_HEIGHT
        } else {
            CANVAS_HEIGHT_IN_TILE * TILE_HEIGHT + actor_at_y - map_height
        }
    }
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
