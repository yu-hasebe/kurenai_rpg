use crate::domain::models::map::{map_id::MapId, Map};

pub trait MapRepository {
    fn new() -> Self;
    fn find(&self, map_id: &MapId) -> Result<Map, String>;
    fn save(&self, map: Map) -> Result<(), String>;
}
