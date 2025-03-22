use backend::domain::filament::{FilamentRepository, FilamentRoll};
use backend::infrastructure::repositories::memory::InMemoryFilamentRepository;

#[test]
fn test_filament_percentage_remaining() {
    // Arrange
    let filament = FilamentRoll {
        id: "test-id".to_string(),
        name: "Test Filament".to_string(),
        material: "PLA".to_string(),
        color: "#000000".to_string(),
        diameter: 1.75,
        weight: 1000.0,
        remaining_weight: 750.0,
        manufacturer: "Test Brand".to_string(),
    };

    // Act
    let percentage = filament.percentage_remaining();

    // Assert
    assert_eq!(percentage, 75.0);
}

#[test]
fn test_update_filament_remaining_weight() {
    // Arrange
    let repository = InMemoryFilamentRepository::new();
    let filament = FilamentRoll {
        id: "test-id-456".to_string(),
        name: "Basic Black PLA".to_string(),
        material: "PLA".to_string(),
        color: "#000000".to_string(),
        diameter: 1.75,
        weight: 1000.0,
        remaining_weight: 1000.0,
        manufacturer: "Hatchbox".to_string(),
    };

    // Save the initial filament
    repository.save(&filament);

    // Act
    repository.update_remaining_weight("test-id-456", 750.0);

    // Get the updated filament
    let updated_filament = repository.find_by_id("test-id-456").unwrap();

    // Assert
    assert_eq!(updated_filament.remaining_weight, 750.0);
    assert_eq!(updated_filament.percentage_remaining(), 75.0);
}
