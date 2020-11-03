pub mod map_id;
pub mod map_repository;

use crate::domain::models::{
    map::map_id::MapId,
    shared::point::{Dot, Point},
};
use derive_new::new;

#[derive(Clone, Debug, new)]
pub struct Map {
    id: MapId,
    size: Point<Dot>,
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Map {}

impl Map {
    pub fn id(&self) -> &MapId {
        &self.id
    }

    pub fn size(&self) -> &Point<Dot> {
        &self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let map1 = Map::new(MapId(0), Point::new(0, 0));
        let map2 = Map::new(MapId(0), Point::new(1, 1));
        let map3 = Map::new(MapId(1), Point::new(0, 0));
        assert_eq!(map2, map1);
        assert_ne!(map3, map1);
    }
}
