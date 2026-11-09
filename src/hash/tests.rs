#[test]
fn sha256() {
    let hash = super::Algorithm::sha256(b"");
    assert_eq!(
        hash,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn sha512() {
    let hash = super::Algorithm::sha512(b"");
    assert_eq!(
        hash,
        "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"
    );
}

#[test]
fn streaming_matches_one_shot_sha256() {
    use super::{Algorithm, StreamingHasher};
    let whole = Algorithm::sha256(b"hello world");
    let streamed = StreamingHasher::new(Algorithm::Sha256)
        .update(b"hello ")
        .update(b"world")
        .finalize_hex();
    assert_eq!(whole, streamed);
    assert_eq!(
        streamed,
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
}

#[test]
fn streaming_matches_one_shot_sha512() {
    use super::{Algorithm, StreamingHasher};
    let whole = Algorithm::sha512(b"hello world");
    let streamed = StreamingHasher::new(Algorithm::Sha512)
        .update(b"hello ")
        .update(b"world")
        .finalize_hex();
    assert_eq!(whole, streamed);
    assert_eq!(
        streamed,
        "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f"
    );
}

#[test]
fn streaming_single_update_matches_one_shot() {
    use super::{Algorithm, StreamingHasher};
    let whole = Algorithm::sha256(b"same");
    let streamed = StreamingHasher::new(Algorithm::Sha256)
        .update(b"same")
        .finalize_hex();
    assert_eq!(whole, streamed);
}

#[test]
fn streaming_hex_and_bytes_agree() {
    use super::{Algorithm, StreamingHasher};
    let digest = StreamingHasher::new(Algorithm::Sha512)
        .update(b"payload")
        .finalize_hex();
    let bytes = StreamingHasher::new(Algorithm::Sha512)
        .update(b"payload")
        .finalize_bytes();
    assert_eq!(digest.len(), 128);
    assert_eq!(bytes.len(), 64);
}

#[test]
fn streaming_empty_state_matches_one_shot() {
    use super::{Algorithm, StreamingHasher};
    let whole = Algorithm::sha256(b"");
    let streamed = StreamingHasher::new(Algorithm::Sha256).finalize_hex();
    assert_eq!(whole, streamed);
}

#[test]
fn StreamingHasher_keeps_state_after_finalizing() {
    use super::{Algorithm, StreamingHasher};
    let mut hasher = StreamingHasher::new(Algorithm::Sha256);
    hasher.update(b"hello ");
    let _ = hasher.finalize_hex();
    hasher.update(b"world");
    let final_digest = hasher.finalize_hex();
    assert_eq!(
        final_digest,
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
}
