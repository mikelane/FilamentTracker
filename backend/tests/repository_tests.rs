// backend/tests/repository_tests.rs
use backend::domain::error::FilamentError;
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

    // Assert
    assert_eq!(updated_filament.remaining_weight, 750.0);
    assert_eq!(updated_filament.percentage_remaining(), 75.0);
}

#[test]
fn test_error_handling_update_negative_weight() {
    // Arrange
    let repository = InMemoryFilamentRepository::new();
    let filament = FilamentRoll {
        id: "test-id-negative".to_string(),
        name: "Test Filament".to_string(),
        material: "PLA".to_string(),
        color: "#000000".to_string(),
        diameter: 1.75,
        weight: 1000.0,
        remaining_weight: 1000.0,
        manufacturer: "Test Brand".to_string(),
    };
    repository.save(&filament);

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
    let filament = FilamentRoll {
        id: "test-id-excessive".to_string(),
        name: "Test Filament".to_string(),
        material: "PLA".to_string(),
        color: "#000000".to_string(),
        diameter: 1.75,
        weight: 1000.0,
        remaining_weight: 1000.0,
        manufacturer: "Test Brand".to_string(),
    };
    repository.save(&filament);

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

    repository.save(&filament1);
    repository.save(&filament2);

    // Act
    let all_filaments = repository.find_all();

    // Assert
    assert_eq!(all_filaments.len(), 2);
    assert!(all_filaments.iter().any(|f| f.id == "test-id-1"));
    assert!(all_filaments.iter().any(|f| f.id == "test-id-2"));
}
