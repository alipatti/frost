use lazy_static::lazy_static;
use rand::thread_rng;
use serde_json::Value;

use crate::*;

lazy_static! {
    pub static ref SECP256K1_SHA256: Value =
        serde_json::from_str(include_str!("tests/vectors.json").trim())
            .expect("Test vector is valid JSON");
}

/// This is testing that Shamir's secret sharing to compute and arbitrary
/// value is working.
#[test]
fn check_share_generation_secp256k1_sha256() {
    let rng = thread_rng();
    frost_core::tests::check_share_generation::<Secp256K1Sha256, _>(rng);
}

#[test]
fn check_sign_with_test_vectors() {
    frost_core::tests::vectors::check_sign_with_test_vectors::<Secp256K1Sha256>(&SECP256K1_SHA256)
}
