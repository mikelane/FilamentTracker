use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FilamentRoll {
    pub id: String,
    pub name: String,
    pub material: String,
    pub color: String,
    pub diameter: f32,
    pub weight: f32,
    pub remaining_weight: f32,
    pub manufacturer: String,
}

impl FilamentRoll {
    pub fn new(
        name: String,
        material: String,
        color: String,
        diameter: f32,
        weight: f32,
        manufacturer: String,
    ) -> Self {
        FilamentRoll {
            id: Uuid::new_v4().to_string(),
            name,
            material,
            color,
            diameter,
            weight,
            remaining_weight: weight, // Initially, remaining weight equals total weight
            manufacturer,
        }
    }

    pub fn percentage_remaining(&self) -> f32 {
        (self.remaining_weight / self.weight) * 100.0
    }
}

pub trait FilamentRepository {
    fn save(&self, filament: &FilamentRoll);
    fn find_by_id(&self, id: &str) -> Option<FilamentRoll>;
    fn update_remaining_weight(&self, id: &str, remaining_weight: f32) -> Option<FilamentRoll>;
}
