use mimir_poseidon2_interop::*

#[test]
fn test_hash_merkle_basic() {
    let left: Field = 1.into();
    let right: Field = 2.into();
    let result = hash_merkle_node(left, right);
    let zero = Field::from(0u64);
    println!("Merkle Node Hash: {}", result);
    assert_ne!(result, zero);

}