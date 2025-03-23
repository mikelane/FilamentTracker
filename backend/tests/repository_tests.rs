use backend::domain::error::FilamentError;
use backend::domain::filament::{FilamentRepository, FilamentRoll};
use backend::infrastructure::repositories::memory::InMemoryFilamentRepository;

// Helper function to create a test filament with ID
fn create_test_filament(id: &str, name: &str, material: &str) -> FilamentRoll {
    FilamentRoll::with_id(
        id.to_string(),
        name.to_string(),
        material.to_string(),
        "#000000".to_string(),
        1.75,
        1000.0,
        1000.0,
        "Test Brand".to_string(),
    )
    .expect("Failed to create test filament")
}

#[test]
fn test_save_filament() {
    let repository = InMemoryFilamentRepository::new();

    let filament = create_test_filament("test-id", "Test Filament", "PLA");

    repository.save(&filament).expect("Failed to save filament");
}

#[test]
fn test_find_filament_by_id() {
    // Arrange
    let repository = InMemoryFilamentRepository::new();
    let filament = create_test_filament("test-id-123", "Test Filament", "PLA");

    // Act
    repository.save(&filament).expect("Failed to save filament");
    let found_filament = repository
        .find_by_id("test-id-123")
        .expect("Failed to find filament");

    // Assert
    assert_eq!(filament.id(), found_filament.id());
    assert_eq!(filament.name(), found_filament.name());
}

#[test]
fn test_error_handling_find_by_id() {
    // Arrange
    let repository = InMemoryFilamentRepository::new();

    // Act
    let result = repository.find_by_id("non-existent-id");

    // Assert
    assert!(result.is_err());
    match result {
        Err(FilamentError::NotFound(id)) => assert_eq!(id, "non-existent-id"),
        _ => panic!("Expected NotFound error"),
    }
}

#[test]
fn test_update_filament_remaining_weight() {
    // Arrange
    let repository = InMemoryFilamentRepository::new();
    let filament = create_test_filament("test-id-456", "Basic Black PLA", "PLA");

    // Save the initial filament
    repository.save(&filament).expect("Failed to save filament");

    // Act
    let updated_filament = repository
        .update_remaining_weight("test-id-456", 750.0)
        .expect("Failed to update weight");

    // Assert
    assert_eq!(updated_filament.remaining_weight(), 750.0);
    assert_eq!(updated_filament.percentage_remaining(), 75.0);
}

#[test]
fn test_error_handling_update_negative_weight() {
    // Arrange
    let repository = InMemoryFilamentRepository::new();
    let filament = create_test_filament("test-id-negative", "Test Filament", "PLA");

    repository.save(&filament).expect("Failed to save filament");

    // Act
    let result = repository.update_remaining_weight("test-id-negative", -10.0);

    // Assert
    assert!(result.is_err());
    match result {
        Err(FilamentError::InvalidData(_)) => assert!(true),
        _ => panic!("Expected InvalidData error"),
    }
}

#[test]
fn test_error_handling_update_excessive_weight() {
    // Arrange
    let repository = InMemoryFilamentRepository::new();
    let filament = create_test_filament("test-id-excessive", "Test Filament", "PLA");

    repository.save(&filament).expect("Failed to save filament");

    // Act
    let result = repository.update_remaining_weight("test-id-excessive", 1200.0);

    // Assert
    assert!(result.is_err());
    match result {
        Err(FilamentError::InvalidData(_)) => assert!(true),
        _ => panic!("Expected InvalidData error"),
    }
}

#[test]
fn test_find_all_filaments() {
    // Arrange
    let repository = InMemoryFilamentRepository::new();

    let filament1 = create_test_filament("test-id-1", "Black PLA", "PLA");
    let filament2 = create_test_filament("test-id-2", "White ABS", "ABS");

    repository
        .save(&filament1)
        .expect("Failed to save filament1");
    repository
        .save(&filament2)
        .expect("Failed to save filament2");

    // Act
    let all_filaments = repository.find_all().expect("Failed to get all filaments");

    // Assert
    assert_eq!(all_filaments.len(), 2);
    assert!(all_filaments.iter().any(|f| f.id() == "test-id-1"));
    assert!(all_filaments.iter().any(|f| f.id() == "test-id-2"));
}

#[test]
fn test_find_by_material() {
    // Arrange
    let repository = InMemoryFilamentRepository::new();

    let filament1 = create_test_filament("test-id-1", "Black PLA", "PLA");
    let filament2 = create_test_filament("test-id-2", "White ABS", "ABS");
    let filament3 = create_test_filament("test-id-3", "Red PLA", "PLA");

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
