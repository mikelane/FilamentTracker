use crate::domain::error::FilamentError;
use crate::domain::filament::{FilamentRepository, FilamentRoll};

pub struct FilamentService<'a> {
    repository: &'a dyn FilamentRepository,
}

impl<'a> FilamentService<'a> {
    pub fn new(repository: &'a dyn FilamentRepository) -> Self {
        FilamentService { repository }
    }

    pub fn get_low_inventory(&self) -> Result<Vec<FilamentRoll>, FilamentError> {
        // Define what "low inventory" means - less than 20% remaining
        const LOW_INVENTORY_THRESHOLD: f32 = 20.0;

        // Get all filaments from the repository
        let all_filaments = self.repository.find_all()?;

        // Filter for filaments with less than 20% remaining
        let low_inventory = all_filaments
            .into_iter()
            .filter(|filament| filament.percentage_remaining() < LOW_INVENTORY_THRESHOLD)
            .collect();

        Ok(low_inventory)
    }
}
