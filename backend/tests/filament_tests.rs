use backend::domain::filament::FilamentRoll;

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