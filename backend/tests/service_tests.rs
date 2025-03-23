use backend::domain::filament::{FilamentRepository, FilamentRoll};
use backend::domain::services::filament_service::FilamentService;
use backend::infrastructure::repositories::memory::InMemoryFilamentRepository;

#[test]
fn test_get_low_inventory_filaments() {
    // Arrange
    let repository = InMemoryFilamentRepository::new();
    let service = FilamentService::new(&repository);

    // Create test filaments with different remaining percentages
    let filament1 = FilamentRoll::with_id(
        "test-id-1".to_string(),
        "Low PLA".to_string(),
        "PLA".to_string(),
        "#FF0000".to_string(),
        1.75,
        1000.0,
        150.0, // 15% remaining
        "Brand A".to_string(),
    )
    .expect("Failed to create test filament");

    let filament2 = FilamentRoll::with_id(
        "test-id-2".to_string(),
        "Medium ABS".to_string(),
        "ABS".to_string(),
        "#FFFFFF".to_string(),
        1.75,
        1000.0,
        500.0, // 50% remaining
        "Brand B".to_string(),
    )
    .expect("Failed to create test filament");

    let filament3 = FilamentRoll::with_id(
        "test-id-3".to_string(),
        "Critical PETG".to_string(),
        "PETG".to_string(),
        "#00FF00".to_string(),
        1.75,
        1000.0,
        50.0, // 5% remaining
        "Brand C".to_string(),
    )
    .expect("Failed to create test filament");

    repository
        .save(&filament1)
        .expect("Failed to save filament");
    repository
        .save(&filament2)
        .expect("Failed to save filament");
    repository
        .save(&filament3)
        .expect("Failed to save filament");

    // Act
    let low_inventory = service
        .get_low_inventory()
        .expect("Failed to get low inventory");

    // Assert
    assert_eq!(low_inventory.len(), 2);
    assert!(low_inventory.iter().any(|f| f.id() == "test-id-1"));
    assert!(low_inventory.iter().any(|f| f.id() == "test-id-3"));
    assert!(!low_inventory.iter().any(|f| f.id() == "test-id-2"));
}
