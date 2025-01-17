fn km_to_fields(km: f64) -> f64 {
    let field_size = 0.01034;
    return (km / field_size).floor();
}

#[test]
fn test_km_to_fields() {
    assert_eq!(km_to_fields(1.034), 100.00)
}
