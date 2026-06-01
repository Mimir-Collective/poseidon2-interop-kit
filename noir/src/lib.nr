// Domains as Defined in Domains.md
pub global DOMAIN_MERKLE_NODE: Field = 0;
pub global DOMAIN_SMT_LEAF: Field = 1;
pub global DOMAIN_NOTE_COMMITMENT: Field = 2;
pub global DOMAIN_NULLIFIER: Field = 3;
pub global DOMAIN_ADDRESS_SALT: Field = 4;

// Raw Poseidon2 Permutation (t=4) wrapper around noir's native poseidon2 implementation
fn permute_3(state: [Field; 4]) -> [Field; 4] {
    std::hash::poseidon2_permutation(state)
}

// Domain-separated hash for fixed length inputs (3 inputs -> t4)
fn hash_3(domain: Field, a: Field, b: Field) -> Field {
    let state = [domain, a, b, 0];
    permute_3(state)[0]
}

// Domain-separated hash for 5 fixed length inputs (5 inputs -> t8)
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

// === Tests ===

#[test]
fn test_hash_merkle_basic() {
    let result = hash_merkle_node(1, 2);
    let expected: Field = 0x2026fba1cf8735645e309b155c86299f3ee23c495e9e94dad868dddb80907730;
    assert(result == expected);
}

#[test]
fn test_hash_merkle_zeros() {
    let result = hash_merkle_node(0, 0);
    let expected: Field = 0x18dfb8dc9b82229cff974efefc8df78b1ce96d9d844236b496785c698bc6732e;
    assert(result == expected);
}

#[test]
fn test_hash_smt_leaf_basic() {
    let result = hash_smt_leaf(3, 4);
    let expected: Field = 0x2b2b0bb3946ec98c2d59ae87c4da6cfbb7f3e779a9ca3853650ba0edbd4d9b06;
    assert(result == expected);
}

#[test]
fn test_hash_note_commitment_basic() {
    let result = hash_note_commitment(5, 6, 7, 8, 9);
    let expected: Field = 0x22e8f91c0b715e9304767df11f256bd18ed79ff0a3a688f5a03d29b5f549ed2c;
    assert(result == expected);
}

#[test]
fn test_hash_nullifier_basic() {
    let result = hash_nullifier(42, 100);
    let expected: Field = 0x298d157938c75d615c29e3f0ec72822ce450611debaa570c2fdd1bbbf0de396c;
    assert(result == expected);
}

#[test]
fn test_hash_address_salt_basic() {
    let result = hash_address_salt(123, 456);
    let expected: Field = 0x07ad23c3e4bc358ec44c744573deec09bb7189282993591e709ef0ab6bc7e801;
    assert(result == expected);
}

#[test]
fn test_domain_separation() {
    let a: Field = 1;
    let b: Field = 2;
    let merkle = hash_merkle_node(a, b);
    let leaf = hash_smt_leaf(a, b);
    let nullifier = hash_nullifier(a, b);
    let address = hash_address_salt(a, b);
    assert(merkle != leaf);
    assert(merkle != nullifier);
    assert(merkle != address);
    assert(leaf != nullifier);
    assert(leaf != address);
    assert(nullifier != address);
}
