// Domains as Defined in Domains.md
pub global DOMAIN_MERKLE_NODE: Field = 0;
pub global DOMAIN_SMT_LEAF: Field = 1;
pub global DOMAIN_NOTE_COMMITMENT: Field = 2;
pub global DOMAIN_NULLIFIER: Field = 3;
pub global DOMAIN_ADDRESS_SALT: Field = 4;

// Raw Poseidon2 Permutation (t=4,t=8) wrapper around noir's native poseidon2 implementation
fn permute_3(state: [Field; 4]) -> [Field; 4] {
    std::hash::poseidon2_permutation(state)
}

// Domain - Seperated hash for fixed length inputs (3 inputs -> t4)
fn hash_3(domain: Field, a: Field, b: Field) -> Field {
    let state = [domain, a, b, 0];
    permute_3(state)[0]
}

// Domain Seperated hash for 5 fixed length inputs (5 inputs -> t8)
fn hash_5(domain: Field, a: Field, b: Field, c: Field, d: Field, e: Field) -> Field {
    let h1 = std::hash::poseidon2_permutation([domain, a, b, c])[0];
    let h2 = std::hash::poseidon2_permutation([h1, d, e, 0])[0];
    h2
}

// Public APIs

pub fn hash_merkle_node(left: Field, right: Field) -> Field {
    hash_3(DOMAIN_MERKLE_NODE, left, right)
}

pub fn hash_smt_leaf(key: Field, value: Field) -> Field {
    hash_3(DOMAIN_SMT_LEAF, key, value)
}

pub fn hash_note_commitment(sk: Field, r: Field, v: Field, token: Field, chain_id: Field) -> Field {
    hash_5(DOMAIN_NOTE_COMMITMENT, sk, r, v, token, chain_id)
}

pub fn hash_nullifier(sk: Field, rho: Field) -> Field {
    hash_3(DOMAIN_NULLIFIER, sk, rho)
}

pub fn hash_address_salt(sk: Field, salt: Field) -> Field {
    hash_3(DOMAIN_ADDRESS_SALT, sk, salt)
}




// tests 

#[test]
fn test_hash_merkle_basic() {
    let left: Field = Field::from(1u64);
    let right: Field = Field::from(2u64);
    let result = hash_merkle_node(left, right);
    let expected_merkle: Field = 0x2026fba1cf8735645e309b155c86299f3ee23c495e9e94dad868dddb80907730;
    let zero = Field::from(0u64);
    assert(result != zero);
    assert(result == expected_merkle);
    println(f"Merkle Node Hash: {result}");
    
}

#[test]
fn test_hash_smt_leaf_basic() {
    let key: Field = Field::from(3u64);
    let value: Field = Field::from(4u64);
    let result = hash_smt_leaf(key, value);
    let expected_smt_leaf: Field = 0x2b2b0bb3946ec98c2d59ae87c4da6cfbb7f3e779a9ca3853650ba0edbd4d9b06; // Rust result in hex form.
    let zero = Field::from(0u64);
    assert(result != zero);
    assert(result == expected_smt_leaf);
}

#[test]
fn test_hash_note_commitment_basic() {
    let sk: Field = Field::from(5u64);
    let r: Field = Field::from(6u64);
    let v: Field = Field::from(7u64);
    let token: Field = Field::from(8u64);
    let chain_id: Field = Field::from(9u64);
    let result = hash_note_commitment(sk, r, v, token, chain_id);
    let expected_note_commitment: Field = 0x22e8f91c0b715e9304767df11f256bd18ed79ff0a3a688f5a03d29b5f549ed2c; // Rust result in hex form.
    let zero = Field::from(0u64);
    assert(result != zero);
    assert(result == expected_note_commitment);
}