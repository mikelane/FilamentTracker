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
    fn save(&self, filament: &FilamentRoll) -> Result<(), FilamentError> {
        let mut filaments = self.filaments.lock().map_err(|e| {
            FilamentError::RepositoryError(format!("Failed to acquire lock: {}", e))
        })?;

        filaments.insert(filament.id().to_string(), filament.clone());
        Ok(())
    }

    fn find_by_id(&self, id: &str) -> Result<FilamentRoll, FilamentError> {
        let filaments = self.filaments.lock().map_err(|e| {
            FilamentError::RepositoryError(format!("Failed to acquire lock: {}", e))
        })?;

        filaments
            .get(id)
            .cloned()
            .ok_or_else(|| FilamentError::NotFound(id.to_string()))
    }

    fn update_remaining_weight(
        &self,
        id: &str,
        remaining_weight: f32,
    ) -> Result<FilamentRoll, FilamentError> {
        let mut filaments = self.filaments.lock().map_err(|e| {
            FilamentError::RepositoryError(format!("Failed to acquire lock: {}", e))
        })?;

        let filament = filaments
            .get_mut(id)
            .ok_or_else(|| FilamentError::NotFound(id.to_string()))?;

        // Use domain entity method for validation and update
        filament.update_remaining_weight(remaining_weight)?;

        Ok(filament.clone())
    }

    fn find_all(&self) -> Result<Vec<FilamentRoll>, FilamentError> {
        let filaments = self.filaments.lock().map_err(|e| {
            FilamentError::RepositoryError(format!("Failed to acquire lock: {}", e))
        })?;

        Ok(filaments.values().cloned().collect())
    }

    fn find_by_material(&self, material: &str) -> Result<Vec<FilamentRoll>, FilamentError> {
        let filaments = self.filaments.lock().map_err(|e| {
            FilamentError::RepositoryError(format!("Failed to acquire lock: {}", e))
        })?;

        Ok(filaments
            .values()
            .filter(|f| f.material() == material)
            .cloned()
            .collect())
    }
}
