#[test]
fn bytes() {
    let bytes = super::bytes(10);
    assert_eq!(bytes.len(), 10);
}

#[test]
fn hex() {
    let len: usize = 10;
    let hex = super::hex(len);
    assert_eq!(hex.len(), len * 2);
}

#[test]
fn range() {
    let begin = 0;
    let end = 10;
    let val = super::range(begin, end);
    assert!(val > begin && val <= end);
}
