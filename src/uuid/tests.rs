#[test]
fn v4() {
    let _ = super::v4();
}

#[test]
fn v5_deterministic() {
    use super::Uuid;
    let a = super::v5(Uuid::NAMESPACE_OID, "cryptox");
    let b = super::v5(Uuid::NAMESPACE_OID, "cryptox");
    assert_eq!(a, b);
}

#[test]
fn v5_known_value() {
    // Verified against python: uuid.uuid5(uuid.NAMESPACE_OID, "cryptox/vector-chunk")
    use super::Uuid;
    let id = super::v5(Uuid::NAMESPACE_OID, "cryptox/vector-chunk");
    assert_eq!(
        id.to_string(),
        "974c3b68-6f95-50e6-a691-616d0c51130f"
    );
}

#[test]
fn v7() {
    let _ = super::v7();
}

#[test]
fn stringify_roundtrip() {
    let id = super::v4();
    let parsed = super::stringify(&id.to_string()).unwrap();
    assert_eq!(id, parsed);
}

#[test]
fn stringify_rejects_garbage() {
    assert!(super::stringify("not-a-uuid").is_err());
}
