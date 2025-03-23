use backend::domain::filament::{FilamentRepository, FilamentRoll, FilamentRollBuilder};
use backend::infrastructure::repositories::memory::InMemoryFilamentRepository;

// Helper function to create a test filament with ID
fn create_test_filament(
    id: &str,
    name: &str,
    material: &str,
    remaining_weight: f32,
) -> FilamentRoll {
    FilamentRollBuilder::new(
        name.to_string(),
        material.to_string(),
        "#000000".to_string(),
        1.75,
        1000.0,
        "Test Brand".to_string(),
    )
    .with_id(id)
    .with_remaining_weight(remaining_weight)
    .build()
    .expect("Failed to create test filament")
}

fn create_test_filament_with_storage(
    id: &str,
    name: &str,
    material: &str,
    remaining_weight: f32,
    storage_location: &str,
) -> FilamentRoll {
    FilamentRollBuilder::new(
        name.to_string(),
        material.to_string(),
        "#000000".to_string(),
        1.75,
        1000.0,
        "Test Brand".to_string(),
    )
    .with_id(id)
    .with_remaining_weight(remaining_weight)
    .with_storage_location(storage_location)
    .build()
    .expect("Failed to create test filament")
}

#[test]
fn test_filament_percentage_remaining() {
    // Arrange
    let filament = create_test_filament("test-id", "Test Filament", "PLA", 750.0);

    // Act
    let percentage = filament.percentage_remaining();

    // Assert
    assert_eq!(percentage, 75.0);
}

#[test]
fn test_update_filament_remaining_weight() {
    // Arrange
    let repository = InMemoryFilamentRepository::new();
    let filament = create_test_filament("test-id-456", "Basic Black PLA", "PLA", 1000.0);

    // Save the initial filament
    repository.save(&filament).expect("Failed to save filament");

    // Act
    let updated_filament = repository
        .update_remaining_weight("test-id-456", 750.0)
        .expect("Failed to update weight");

    // Get the updated filament
    let retrieved_filament = repository
        .find_by_id("test-id-456")
        .expect("Failed to retrieve filament");

    // Assert
    assert_eq!(updated_filament.remaining_weight(), 750.0);
    assert_eq!(updated_filament.percentage_remaining(), 75.0);
    assert_eq!(retrieved_filament.remaining_weight(), 750.0);
    assert_eq!(retrieved_filament.percentage_remaining(), 75.0);
}

#[test]
fn test_find_by_material() {
    // Arrange
    let repository = InMemoryFilamentRepository::new();

    let filament1 = create_test_filament("test-id-1", "Black PLA", "PLA", 1000.0);
    let filament2 = create_test_filament("test-id-2", "White ABS", "ABS", 750.0);
    let filament3 = create_test_filament("test-id-3", "Red PLA", "PLA", 800.0);

    repository
        .save(&filament1)
        .expect("Failed to save filament1");
    repository
        .save(&filament2)
        .expect("Failed to save filament2");
    repository
        .save(&filament3)
        .expect("Failed to save filament3");

    // Act
    let pla_filaments = repository
        .find_by_material("PLA")
        .expect("Failed to find by material");

    // Assert
    assert_eq!(pla_filaments.len(), 2);
    assert!(pla_filaments.iter().all(|f| f.material() == "PLA"));
    assert!(pla_filaments.iter().any(|f| f.id() == "test-id-1"));
    assert!(pla_filaments.iter().any(|f| f.id() == "test-id-3"));
}

#[test]
fn test_create_filament_with_storage_location() {
    // Arrange
    let storage_location = "Blue Box";

    // Act
    let filament = FilamentRollBuilder::new(
        "Test Filament".to_string(),
        "PLA".to_string(),
        "#000000".to_string(),
        1.75,
        1000.0,
        "Test Brand".to_string(),
    )
    .with_id("test-id")
    .with_remaining_weight(1000.0)
    .with_storage_location(storage_location)
    .build();

    // Assert
    assert!(filament.is_ok());
    let filament = filament.unwrap();
    assert_eq!(filament.storage_location(), storage_location);
}

#[test]
fn test_filament_without_storage_location() {
    // Arrange & Act
    let filament = FilamentRollBuilder::new(
        "Test Filament".to_string(),
        "PLA".to_string(),
        "#000000".to_string(),
        1.75,
        1000.0,
        "Test Brand".to_string(),
    )
    .build()
    .unwrap();

    // Assert
    assert_eq!(filament.storage_location(), "");
}
