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
    // Keep fields private
    id: String,
    name: String,
    material: String,
    color: String,
    diameter: f32,
    weight: f32,
    remaining_weight: f32,
    manufacturer: String,
}

impl FilamentRoll {
    pub fn new(
        name: String,
        material: String,
        color: String,
        diameter: f32,
        weight: f32,
        manufacturer: String,
    ) -> Result<Self, FilamentError> {
        // Validate inputs in the domain entity
        if name.is_empty() {
            return Err(FilamentError::InvalidData(
                "Name cannot be empty".to_string(),
            ));
        }

        if material.is_empty() {
            return Err(FilamentError::InvalidData(
                "Material cannot be empty".to_string(),
            ));
        }

        if color.is_empty() {
            return Err(FilamentError::InvalidData(
                "Color cannot be empty".to_string(),
            ));
        }

        if diameter <= 0.0 {
            return Err(FilamentError::InvalidData(
                "Diameter must be positive".to_string(),
            ));
        }

        if weight <= 0.0 {
            return Err(FilamentError::InvalidData(
                "Weight must be positive".to_string(),
            ));
        }

        if manufacturer.is_empty() {
            return Err(FilamentError::InvalidData(
                "Manufacturer cannot be empty".to_string(),
            ));
        }

        Ok(FilamentRoll {
            id: Uuid::new_v4().to_string(),
            name,
            material,
            color,
            diameter,
            weight,
            remaining_weight: weight, // Initially, remaining weight equals total weight
            manufacturer,
        })
    }

    // Factory method for creating from existing ID (for tests and repositories)
    pub fn with_id(
        id: String,
        name: String,
        material: String,
        color: String,
        diameter: f32,
        weight: f32,
        remaining_weight: f32,
        manufacturer: String,
    ) -> Result<Self, FilamentError> {
        // Validate inputs
        if id.is_empty() {
            return Err(FilamentError::InvalidData("ID cannot be empty".to_string()));
        }

        if name.is_empty() {
            return Err(FilamentError::InvalidData(
                "Name cannot be empty".to_string(),
            ));
        }

        if material.is_empty() {
            return Err(FilamentError::InvalidData(
                "Material cannot be empty".to_string(),
            ));
        }

        if color.is_empty() {
            return Err(FilamentError::InvalidData(
                "Color cannot be empty".to_string(),
            ));
        }

        if diameter <= 0.0 {
            return Err(FilamentError::InvalidData(
                "Diameter must be positive".to_string(),
            ));
        }

        if weight <= 0.0 {
            return Err(FilamentError::InvalidData(
                "Weight must be positive".to_string(),
            ));
        }

        // Validate remaining_weight against total weight
        if remaining_weight < 0.0 {
            return Err(FilamentError::InvalidData(
                "Remaining weight cannot be negative".to_string(),
            ));
        }

        if remaining_weight > weight {
            return Err(FilamentError::InvalidData(
                "Remaining weight cannot exceed total weight".to_string(),
            ));
        }

        if manufacturer.is_empty() {
            return Err(FilamentError::InvalidData(
                "Manufacturer cannot be empty".to_string(),
            ));
        }

        Ok(FilamentRoll {
            id,
            name,
            material,
            color,
            diameter,
            weight,
            remaining_weight,
            manufacturer,
        })
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
}
