use crate::domain::error::FilamentError;
use crate::domain::filament::{FilamentRepository, FilamentRoll};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct InMemoryFilamentRepository {
    filaments: Arc<Mutex<HashMap<String, FilamentRoll>>>,
}

impl Default for InMemoryFilamentRepository {
    fn default() -> Self {
        Self {
            filaments: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl InMemoryFilamentRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FilamentRepository for InMemoryFilamentRepository {
    fn save(&self, filament: &FilamentRoll) {
        let mut filaments = self.filaments.lock().unwrap();
        filaments.insert(filament.id.clone(), filament.clone());
    }

    fn find_by_id(&self, id: &str) -> Result<FilamentRoll, FilamentError> {
        let filaments = self.filaments.lock().unwrap();

        match filaments.get(id) {
            Some(filament) => Ok(filament.clone()),
            None => Err(FilamentError::NotFound(id.to_string())),
        }
    }

    fn update_remaining_weight(
        &self,
        id: &str,
        remaining_weight: f32,
    ) -> Result<FilamentRoll, FilamentError> {
        if remaining_weight < 0.0 {
            return Err(FilamentError::InvalidData(
                "Remaining weight cannot be negative".to_string(),
            ));
        }

        let mut filaments = self.filaments.lock().unwrap();

        match filaments.get_mut(id) {
            Some(filament) => {
                if remaining_weight > filament.weight {
                    return Err(FilamentError::InvalidData(
                        "Remaining weight cannot exceed total weight".to_string(),
                    ));
                }

                filament.remaining_weight = remaining_weight;
                Ok(filament.clone())
            }
            None => Err(FilamentError::NotFound(id.to_string())),
        }
    }

    fn find_all(&self) -> Vec<FilamentRoll> {
        let filaments = self.filaments.lock().unwrap();
        filaments.values().cloned().collect()
    }

    fn find_by_material(&self, material: &str) -> Vec<FilamentRoll> {
        let filaments = self.filaments.lock().unwrap();
        filaments
            .values()
            .filter(|f| f.material == material)
            .cloned()
            .collect()
    }
}
