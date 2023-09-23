use pqc_dilithium::*;

#[cfg(feature = "mode2")]
#[test]
fn mode2_sign_then_verify_valid() {
  let msg = b"Hello";
  let keys =mode2::api:: Keypair::generate();
  let signature = keys.sign(msg);
  assert!(mode2::api::verify(&signature, msg, &keys.public).is_ok())
}

#[cfg(feature = "mode2")]
#[test]
fn mode2_sign_then_verify_invalid() {
  let msg = b"Hello";
  let keys = mode2::api::Keypair::generate();
  let mut signature = keys.sign(msg);
  signature[..4].copy_from_slice(&[255u8; 4]);
  assert!(mode2::api::verify(&signature, msg, &keys.public).is_err())
}

#[cfg(feature = "mode3")]
#[test]
fn mode3_sign_then_verify_valid() {
  let msg = b"Hello";
  let keys = mode3::api::Keypair::generate();
  let signature = keys.sign(msg);
  assert!(mode3::api::verify(&signature, msg, &keys.public).is_ok())
}

#[cfg(feature = "mode3")]
#[test]
fn mode3_sign_then_verify_invalid() {
  let msg = b"Hello";
  let keys = mode3::api::Keypair::generate();
  let mut signature = keys.sign(msg);
  signature[..4].copy_from_slice(&[255u8; 4]);
  assert!(mode3::api::verify(&signature, msg, &keys.public).is_err())
}

#[cfg(feature = "mode5")]
#[test]
fn mode5_sign_then_verify_valid() {
  let msg = b"Hello";
  let keys = mode5::api::Keypair::generate();
  let signature = keys.sign(msg);
  assert!(mode5::api::verify(&signature, msg, &keys.public).is_ok())
}

#[cfg(feature = "mode5")]
#[test]
fn mode5_sign_then_verify_invalid() {
  let msg = b"Hello";
  let keys = mode5::api::Keypair::generate();
  let mut signature = keys.sign(msg);
  signature[..4].copy_from_slice(&[255u8; 4]);
  assert!(mode5::api::verify(&signature, msg, &keys.public).is_err())
}
