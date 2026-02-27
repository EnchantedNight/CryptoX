#[cfg(feature = "uuid")]
pub mod uuid;
#[cfg(feature = "bcrypt")]
pub mod bcrypt;
#[cfg(feature = "hash")]
pub mod hash;

#[cfg(not(feature = "hash"))]
pub fn hash_not_available() {
    compile_error!("Hash module requires 'hash' feature");
}