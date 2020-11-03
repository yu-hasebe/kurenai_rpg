use crate::domain::models::scene::Scene;

pub trait SceneRepository {
    fn new(scene: Scene) -> Self;
    fn find(&self) -> Result<Scene, String>;
    fn save(&self, scene: Scene) -> Result<(), String>;
}
