//! ### UUID module providers function to generate universal unique identifier version 4 and 7

#[cfg(not(feature = "uuid"))]
pub fn not_available() {
    compile_error!("UUID module requires 'uuid' feature");
}

use uuid::Uuid;

/// Generates universal unique identifier version 4
///
/// # Examples
///
/// ```
/// use cryptox::uuid;
/// let id = uuid::v4();
/// println!("Id is - {}", id);
/// ```
pub fn v4() -> Uuid {
    Uuid::new_v4()
}

/// Generates universal unique identifier version 7
///
/// # Examples
///
/// ```
/// use cryptox::uuid;
/// let id = uuid::v7();
/// println!("Id is - {}", id);
/// ```
pub fn v7() -> Uuid {
    Uuid::now_v7()
}