pub mod map_id;

use crate::domain::models::{map::map_id::MapId, shared::game_point::GamePoint};
use derive_new::new;
use kurenai::point::{Dot, Point};

#[derive(Clone, Debug, new)]
pub struct Map {
    id: MapId,
    size: GamePoint<Dot>,
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.map_id() == other.map_id()
    }
}

impl Eq for Map {}

impl Map {
    fn map_id(&self) -> &MapId {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let map1 = Map::new(MapId(0), GamePoint::new(0, 0));
        let map2 = Map::new(MapId(0), GamePoint::new(1, 1));
        let map3 = Map::new(MapId(1), GamePoint::new(0, 0));
        assert_eq!(map2, map1);
        assert_ne!(map3, map1);
    }
}
