use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use mimir_poseidon2_interop::*;

fn from_hex(s: &str) -> Fr {
    Fr::from_be_bytes_mod_order(&hex::decode(&s[2..]).unwrap())
}

// === Hardcoded expected values (from Rust reference, cross-checked with Noir) ===

#[test]
fn test_hash_merkle_node_basic() {
    let left = Fr::from(1u64);
    let right = Fr::from(2u64);
    let result = hash_merkle_node(left, right);
    let expected = from_hex("0x2026fba1cf8735645e309b155c86299f3ee23c495e9e94dad868dddb80907730");
    assert_eq!(result, expected, "hash_merkle_node(1,2) mismatch");
}

#[test]
fn test_hash_merkle_node_zeros() {
    let result = hash_merkle_node(Fr::from(0u64), Fr::from(0u64));
    let expected = from_hex("0x18dfb8dc9b82229cff974efefc8df78b1ce96d9d844236b496785c698bc6732e");
    assert_eq!(result, expected, "hash_merkle_node(0,0) mismatch");
}

#[test]
fn test_hash_smt_leaf_basic() {
    let key = Fr::from(3u64);
    let value = Fr::from(4u64);
    let result = hash_smt_leaf(key, value);
    let expected = from_hex("0x2b2b0bb3946ec98c2d59ae87c4da6cfbb7f3e779a9ca3853650ba0edbd4d9b06");
    assert_eq!(result, expected, "hash_smt_leaf(3,4) mismatch");
}

#[test]
fn test_hash_note_commitment_basic() {
    let result = hash_note_commitment(Fr::from(5u64), Fr::from(6u64), Fr::from(7u64), Fr::from(8u64), Fr::from(9u64));
    let expected = from_hex("0x22e8f91c0b715e9304767df11f256bd18ed79ff0a3a688f5a03d29b5f549ed2c");
    assert_eq!(result, expected, "hash_note_commitment(5,6,7,8,9) mismatch");
}

#[test]
fn test_hash_nullifier_basic() {
    let result = hash_nullifier(Fr::from(42u64), Fr::from(100u64));
    let expected = from_hex("0x298d157938c75d615c29e3f0ec72822ce450611debaa570c2fdd1bbbf0de396c");
    assert_eq!(result, expected, "hash_nullifier(42,100) mismatch");
}

#[test]
fn test_hash_address_salt_basic() {
    let result = hash_address_salt(Fr::from(123u64), Fr::from(456u64));
    let expected = from_hex("0x07ad23c3e4bc358ec44c744573deec09bb7189282993591e709ef0ab6bc7e801");
    assert_eq!(result, expected, "hash_address_salt(123,456) mismatch");
}

#[test]
fn test_domain_separation() {
    let a = Fr::from(1u64);
    let b = Fr::from(2u64);
    let merkle = hash_merkle_node(a, b);
    let leaf = hash_smt_leaf(a, b);
    let nullifier = hash_nullifier(a, b);
    let address = hash_address_salt(a, b);
    assert_ne!(merkle, leaf, "Merkle node and SMT leaf should differ");
    assert_ne!(merkle, nullifier, "Merkle node and Nullifier should differ");
    assert_ne!(merkle, address, "Merkle node and Address salt should differ");
    assert_ne!(leaf, nullifier, "SMT leaf and Nullifier should differ");
    assert_ne!(leaf, address, "SMT leaf and Address salt should differ");
    assert_ne!(nullifier, address, "Nullifier and Address salt should differ");
}

#[test]
fn test_determinism() {
    let a = Fr::from(7u64);
    let b = Fr::from(13u64);
    let r1 = hash_merkle_node(a, b);
    let r2 = hash_merkle_node(a, b);
    assert_eq!(r1, r2, "Same inputs must produce same output");
}

#[test]
fn test_bytes_to_field_valid() {
    let bytes = [0u8; 32];
    assert!(bytes_to_field(&bytes).is_ok());
    let mut bytes = [0u8; 32];
    bytes[31] = 1;
    assert!(bytes_to_field(&bytes).is_ok());
}

#[test]
fn test_bytes_to_field_invalid_overflow() {
    let overflow_bytes = [0xffu8; 32];
    assert!(bytes_to_field(&overflow_bytes).is_err(), "Should reject bytes >= p");
}

#[test]
fn test_bytes_to_field_p_minus_1() {
    // p-1 for BN254 scalar field
    let bytes: [u8; 32] = hex::decode("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000")
        .unwrap()
        .try_into()
        .unwrap();
    assert!(bytes_to_field(&bytes).is_ok(), "p-1 should be valid");
}

// === JSON fixture loading tests ===

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Vector {
    name: String,
    function: String,
    inputs: Vec<String>,
    expected_output: serde_json::Value,
}

fn load_vectors(path: &str) -> Vec<Vector> {
    let content = fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read {}", path));
    serde_json::from_str(&content).unwrap()
}

#[test]
fn test_vectors_merkle_node() {
    let vectors = load_vectors("../vectors/merkle_node.json");
    for v in vectors {
        let left = from_hex(&v.inputs[0]);
        let right = from_hex(&v.inputs[1]);
        let result = hash_merkle_node(left, right);
        let expected = from_hex(v.expected_output.as_str().unwrap());
        assert_eq!(result, expected, "Vector '{}' failed", v.name);
    }
}

#[test]
fn test_vectors_nullifier() {
    let vectors = load_vectors("../vectors/nullifier.json");
    for v in vectors {
        let sk = from_hex(&v.inputs[0]);
        let rho = from_hex(&v.inputs[1]);
        let result = hash_nullifier(sk, rho);
        let expected = from_hex(v.expected_output.as_str().unwrap());
        assert_eq!(result, expected, "Vector '{}' failed", v.name);
    }
}

#[test]
fn test_vectors_note_commitment() {
    let vectors = load_vectors("../vectors/note_commitment.json");
    for v in vectors {
        let result = hash_note_commitment(
            from_hex(&v.inputs[0]),
            from_hex(&v.inputs[1]),
            from_hex(&v.inputs[2]),
            from_hex(&v.inputs[3]),
            from_hex(&v.inputs[4]),
        );
        let expected = from_hex(v.expected_output.as_str().unwrap());
        assert_eq!(result, expected, "Vector '{}' failed", v.name);
    }
}

#[test]
fn test_vectors_address_salt() {
    let vectors = load_vectors("../vectors/address_salt.json");
    for v in vectors {
        let result = hash_address_salt(from_hex(&v.inputs[0]), from_hex(&v.inputs[1]));
        let expected = from_hex(v.expected_output.as_str().unwrap());
        assert_eq!(result, expected, "Vector '{}' failed", v.name);
    }
}
