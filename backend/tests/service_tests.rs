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
        "test-id-1",
        "Low PLA",
        "PLA",
        "#FF0000",
        1.75,
        1000.0,
        150.0, // 15% remaining
        "Brand A",
        "Bin 1",
    )
    .expect("Failed to create test filament");

    let filament2 = FilamentRoll::with_id(
        "test-id-2",
        "Medium ABS",
        "ABS",
        "#FFFFFF",
        1.75,
        1000.0,
        500.0, // 50% remaining
        "Brand B",
        "Bin 2",
    )
    .expect("Failed to create test filament");

    let filament3 = FilamentRoll::with_id(
        "test-id-3",
        "Critical PETG",
        "PETG",
        "#00FF00",
        1.75,
        1000.0,
        50.0, // 5% remaining
        "Brand C",
        "Bin 3",
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
