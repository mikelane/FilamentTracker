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
    let updated_filament = repository
        .update_remaining_weight("test-id-456", 750.0)
        .unwrap();

    // Get the updated filament
    let retrieved_filament = repository.find_by_id("test-id-456").unwrap();

    // Assert
    assert_eq!(updated_filament.remaining_weight, 750.0);
    assert_eq!(updated_filament.percentage_remaining(), 75.0);
    assert_eq!(retrieved_filament.remaining_weight, 750.0);
    assert_eq!(retrieved_filament.percentage_remaining(), 75.0);
}

#[test]
fn test_find_by_material() {
    // Arrange
    let repository = InMemoryFilamentRepository::new();

    let filament1 = FilamentRoll {
        id: "test-id-1".to_string(),
        name: "Black PLA".to_string(),
        material: "PLA".to_string(),
        color: "#000000".to_string(),
        diameter: 1.75,
        weight: 1000.0,
        remaining_weight: 1000.0,
        manufacturer: "Brand A".to_string(),
    };

    let filament2 = FilamentRoll {
        id: "test-id-2".to_string(),
        name: "White ABS".to_string(),
        material: "ABS".to_string(),
        color: "#FFFFFF".to_string(),
        diameter: 1.75,
        weight: 1000.0,
        remaining_weight: 750.0,
        manufacturer: "Brand B".to_string(),
    };

    let filament3 = FilamentRoll {
        id: "test-id-3".to_string(),
        name: "Red PLA".to_string(),
        material: "PLA".to_string(),
        color: "#FF0000".to_string(),
        diameter: 1.75,
        weight: 1000.0,
        remaining_weight: 800.0,
        manufacturer: "Brand C".to_string(),
    };

    repository.save(&filament1);
    repository.save(&filament2);
    repository.save(&filament3);

    // Act
    let pla_filaments = repository.find_by_material("PLA");

    // Assert
    assert_eq!(pla_filaments.len(), 2);
    assert!(pla_filaments.iter().all(|f| f.material == "PLA"));
    assert!(pla_filaments.iter().any(|f| f.id == "test-id-1"));
    assert!(pla_filaments.iter().any(|f| f.id == "test-id-3"));
}
