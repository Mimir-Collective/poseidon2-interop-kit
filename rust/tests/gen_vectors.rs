use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use mimir_poseidon2_interop::*;

fn to_hex(f: Fr) -> String {
    let bigint = f.into_bigint();
    let bytes = bigint.to_bytes_be();
    format!("0x{:0>64}", hex::encode(bytes))
}

#[test]
fn print_vectors() {
    // Merkle node
    println!("=== merkle_node ===");
    println!("hash_merkle_node(1,2) = {}", to_hex(hash_merkle_node(Fr::from(1u64), Fr::from(2u64))));
    println!("hash_merkle_node(0,0) = {}", to_hex(hash_merkle_node(Fr::from(0u64), Fr::from(0u64))));

    // SMT leaf
    println!("=== smt_leaf ===");
    println!("hash_smt_leaf(3,4) = {}", to_hex(hash_smt_leaf(Fr::from(3u64), Fr::from(4u64))));
    println!("hash_smt_leaf(0,0) = {}", to_hex(hash_smt_leaf(Fr::from(0u64), Fr::from(0u64))));

    // Note commitment
    println!("=== note_commitment ===");
    println!("hash_note_commitment(5,6,7,8,9) = {}", to_hex(hash_note_commitment(Fr::from(5u64), Fr::from(6u64), Fr::from(7u64), Fr::from(8u64), Fr::from(9u64))));

    // Nullifier
    println!("=== nullifier ===");
    println!("hash_nullifier(42,100) = {}", to_hex(hash_nullifier(Fr::from(42u64), Fr::from(100u64))));
    println!("hash_nullifier(1,1) = {}", to_hex(hash_nullifier(Fr::from(1u64), Fr::from(1u64))));

    // Address salt
    println!("=== address_salt ===");
    println!("hash_address_salt(123,456) = {}", to_hex(hash_address_salt(Fr::from(123u64), Fr::from(456u64))));

    // Raw permutation
    println!("=== raw_permutation ===");
    let state = [Fr::from(0u64), Fr::from(1u64), Fr::from(2u64), Fr::from(3u64)];
    let result = taceo_poseidon2::bn254::t4::permutation(&state);
    for (i, r) in result.iter().enumerate() {
        println!("permute([0,1,2,3])[{}] = {}", i, to_hex(*r));
    }
}
