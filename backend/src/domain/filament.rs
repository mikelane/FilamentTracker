use crate::domain::error::FilamentError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait FilamentRepository {
    // Add error handling to save operation
    fn save(&self, filament: &FilamentRoll) -> Result<(), FilamentError>;

    // These methods already return Results which is good
    fn find_by_id(&self, id: &str) -> Result<FilamentRoll, FilamentError>;
    fn update_remaining_weight(
        &self,
        id: &str,
        remaining_weight: f32,
    ) -> Result<FilamentRoll, FilamentError>;

    // Return Results for collection methods too - they could fail
    fn find_all(&self) -> Result<Vec<FilamentRoll>, FilamentError>;
    fn find_by_material(&self, material: &str) -> Result<Vec<FilamentRoll>, FilamentError>;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FilamentRoll {
    // Core attributes
    id: String,
    name: String,
    material: String,
    color: String,
    diameter: f32,
    weight: f32,
    remaining_weight: f32,
    manufacturer: String,

    // Optional attributes
    storage_location: Option<String>,
}

// Builder pattern for FilamentRoll construction
pub struct FilamentRollBuilder {
    id: Option<String>,
    name: String,
    material: String,
    color: String,
    diameter: f32,
    weight: f32,
    remaining_weight: Option<f32>,
    manufacturer: String,
    storage_location: Option<String>,
}

impl FilamentRollBuilder {
    pub fn new(
        name: String,
        material: String,
        color: String,
        diameter: f32,
        weight: f32,
        manufacturer: String,
    ) -> Self {
        FilamentRollBuilder {
            id: None,
            name,
            material,
            color,
            diameter,
            weight,
            remaining_weight: None,
            manufacturer,
            storage_location: None,
        }
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn with_remaining_weight(mut self, remaining_weight: f32) -> Self {
        self.remaining_weight = Some(remaining_weight);
        self
    }

    pub fn with_storage_location(mut self, storage_location: &str) -> Self {
        self.storage_location = Some(storage_location.to_string());
        self
    }

    pub fn build(self) -> Result<FilamentRoll, FilamentError> {
        // Validate required fields
        if self.name.is_empty() {
            return Err(FilamentError::InvalidData(
                "Name cannot be empty".to_string(),
            ));
        }

        if self.material.is_empty() {
            return Err(FilamentError::InvalidData(
                "Material cannot be empty".to_string(),
            ));
        }

        if self.color.is_empty() {
            return Err(FilamentError::InvalidData(
                "Color cannot be empty".to_string(),
            ));
        }

        if self.diameter <= 0.0 {
            return Err(FilamentError::InvalidData(
                "Diameter must be positive".to_string(),
            ));
        }

        if self.weight <= 0.0 {
            return Err(FilamentError::InvalidData(
                "Weight must be positive".to_string(),
            ));
        }

        if self.manufacturer.is_empty() {
            return Err(FilamentError::InvalidData(
                "Manufacturer cannot be empty".to_string(),
            ));
        }

        // For id, use provided or generate new UUID
        let id = self.id.unwrap_or_else(|| Uuid::new_v4().to_string());

        // For remaining_weight, use provided or set to total weight
        let remaining_weight = self.remaining_weight.unwrap_or(self.weight);

        // Validate remaining_weight against total weight
        if remaining_weight < 0.0 {
            return Err(FilamentError::InvalidData(
                "Remaining weight cannot be negative".to_string(),
            ));
        }

        if remaining_weight > self.weight {
            return Err(FilamentError::InvalidData(
                "Remaining weight cannot exceed total weight".to_string(),
            ));
        }

        Ok(FilamentRoll {
            id,
            name: self.name,
            material: self.material,
            color: self.color,
            diameter: self.diameter,
            weight: self.weight,
            remaining_weight,
            manufacturer: self.manufacturer,
            storage_location: self.storage_location,
        })
    }
}

impl FilamentRoll {
    // Factory methods
    pub fn new(
        name: String,
        material: String,
        color: String,
        diameter: f32,
        weight: f32,
        manufacturer: String,
    ) -> Result<Self, FilamentError> {
        FilamentRollBuilder::new(name, material, color, diameter, weight, manufacturer).build()
    }

    pub fn with_id(
        id: &str,
        name: &str,
        material: &str,
        color: &str,
        diameter: f32,
        weight: f32,
        remaining_weight: f32,
        manufacturer: &str,
        storage_location: &str,
    ) -> Result<Self, FilamentError> {
        FilamentRollBuilder::new(
            name.to_string(),
            material.to_string(),
            color.to_string(),
            diameter,
            weight,
            manufacturer.to_string(),
        )
        .with_id(id)
        .with_remaining_weight(remaining_weight)
        .with_storage_location(storage_location)
        .build()
    }

    pub fn update_remaining_weight(&mut self, new_weight: f32) -> Result<(), FilamentError> {
        if new_weight < 0.0 {
            return Err(FilamentError::InvalidData(
                "Remaining weight cannot be negative".to_string(),
            ));
        }

        if new_weight > self.weight {
            return Err(FilamentError::InvalidData(
                "Remaining weight cannot exceed total weight".to_string(),
            ));
        }

        self.remaining_weight = new_weight;
        Ok(())
    }

    pub fn percentage_remaining(&self) -> f32 {
        // Guard against division by zero
        if self.weight == 0.0 {
            return 0.0;
        }

        (self.remaining_weight / self.weight) * 100.0
    }

    // Getters
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn material(&self) -> &str {
        &self.material
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn diameter(&self) -> f32 {
        self.diameter
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn remaining_weight(&self) -> f32 {
        self.remaining_weight
    }

    pub fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    pub fn storage_location(&self) -> &str {
        self.storage_location.as_deref().unwrap_or("")
    }
}
