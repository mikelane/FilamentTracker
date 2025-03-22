// backend/src/infrastructure/repositories/memory.rs
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

    fn find_by_id(&self, id: &str) -> Option<FilamentRoll> {
        let filaments = self.filaments.lock().unwrap();
        filaments.get(id).cloned()
    }

    fn update_remaining_weight(&self, id: &str, remaining_weight: f32) -> Option<FilamentRoll> {
        let mut filaments = self.filaments.lock().unwrap();

        if let Some(filament) = filaments.get_mut(id) {
            filament.remaining_weight = remaining_weight;
            return Some(filament.clone());
        }

        None
    }
}
