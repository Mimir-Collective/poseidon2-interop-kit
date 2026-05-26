use ark_bn254::Fr;
use mimir_poseidon2_interop::*;
use taceo_poseidon2::bn254::t4;


#[test]
fn test_padding_nonzero() {
    let state = [Fr::from(0u64), Fr::from(1u64), Fr::from(2u64), Fr::from(1u64)];
    let result = taceo_poseidon2::bn254::t4::permutation(&state);
    println!("[0,1,2,1] output[0] = {:?}", result[0]);
}

#[test]
fn test_raw_permutation() {
    let state = [Fr::from(0u64), Fr::from(1u64), Fr::from(2u64), Fr::from(0u64)];
    let result = t4::permutation(&state);
    println!("raw permute [0,1,2,0] = {:?}", result[0]);
}


#[test]
fn test_hash_merkle_node_basic() {
let left = Fr::from(1u64);
let right = Fr::from(2u64);
let result = hash_merkle_node(left, right);
println!("hash_merkle_node(1, 2) = {}",result);
assert_ne!(result, Fr::from(0u64), "Hash should not be zero");
}

#[test]
fn test_hash_smt_leaf_basic() {
// test to make sure output is not zero for non-zero inputs
let key = Fr::from(3u64);
let value = Fr::from(4u64);
let result = hash_smt_leaf(key, value);
println!("hash_smt_leaf(3, 4) = {}",result);
assert_ne!(result, Fr::from(0u64), "Hash should not be zero");

}

#[test]
fn test_hash_note_commitment_basic() {
// test to make sure output is not zero for non-zero inputs
let sk = Fr::from(5u64);
let r = Fr::from(6u64);
let v = Fr::from(7u64);
let token = Fr::from(8u64);
let chain_id = Fr::from(9u64);
let result = hash_note_commitment(sk, r, v, token, chain_id);
println!("hash_note_commitment(5,6,7,8,9) = {}",result);
assert_ne!(result, Fr::from(0u64), "Hash should not be zero");
}

#[test]
fn test_hash_nullifier_basic() {
let sk = Fr::from(42u64);
let rho = Fr::from(100u64);
let result = hash_nullifier(sk, rho);
println!("hash_nullifier(42, 100) = {}",result);
assert_ne!(result, Fr::from(0u64), "Hash should not be zero");
}

#[test]
fn test_hash_address_salt_basic() {
let sk = Fr::from(123u64);
let salt = Fr::from(456u64);
let result = hash_address_salt(sk, salt);
println!("hash_address_salt(123, 456) = {}",result);
assert_ne!(result, Fr::from(0u64), "Hash should not be zero");
}

#[test]
fn test_domain_separation() {
    let a = Fr::from(1u64);
    let b = Fr::from(2u64);
    let merkle = hash_merkle_node(a, b);
    let leaf = hash_smt_leaf(a, b);
    let nullifier = hash_nullifier(a, b);
    assert_ne!(merkle, leaf, "Merkle node and SMT leaf should differ");
    assert_ne!(merkle, nullifier, "Merkle node and Nullifier should differ");
    assert_ne!(leaf, nullifier, "SMT leaf and Nullifier should differ");
}

#[test]
fn test_bytes_to_field_valid() {
    // check zero is valid
    let bytes = [0u8; 32];
    assert!(bytes_to_field(&bytes).is_ok());
    // check one is valid
    let mut bytes = [0u8; 32];
    bytes[31] = 1;
    assert!(bytes_to_field(&bytes).is_ok());
    // check p-1 is valid
    let bytes: [u8; 32] = hex::decode("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000")
    .unwrap()
    .try_into()
    .unwrap();
     assert!(bytes_to_field(&bytes).is_ok());
 }

#[test]
fn test_bytes_to_field_invalid_overflow() {
    let overflow_bytes = [0xffu8; 32];
    assert!(bytes_to_field(&overflow_bytes).is_err(), "Should reject bytes >= p");
}