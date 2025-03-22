// backend/tests/repository_tests.rs (update existing file)
use backend::domain::filament::{FilamentRepository, FilamentRoll};
use backend::infrastructure::repositories::memory::InMemoryFilamentRepository;

#[test]
fn test_save_filament() {
    let repository = InMemoryFilamentRepository::new();

    let filament = FilamentRoll {
        id: "test-id".to_string(),
        name: "Test Filament".to_string(),
        material: "PLA".to_string(),
        color: "#000000".to_string(),
        diameter: 1.75,
        weight: 1000.0,
        remaining_weight: 1000.0,
        manufacturer: "Test Brand".to_string(),
    };

    repository.save(&filament);
}

#[test]
fn test_find_filament_by_id() {
    // Arrange
    let repository = InMemoryFilamentRepository::new();
    let filament = FilamentRoll {
        id: "test-id-123".to_string(),
        name: "Test Filament".to_string(),
        material: "PLA".to_string(),
        color: "#000000".to_string(),
        diameter: 1.75,
        weight: 1000.0,
        remaining_weight: 1000.0,
        manufacturer: "Test Brand".to_string(),
    };

    // Act
    repository.save(&filament);
    let found_filament = repository.find_by_id("test-id-123").unwrap();

    // Assert
    assert_eq!(filament.id, found_filament.id);
    assert_eq!(filament.name, found_filament.name);
}
