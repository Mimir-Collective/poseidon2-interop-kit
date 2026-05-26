use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};

// Domains from domains.md specs
pub const DOMAIN_MERKLE_NODE: u64 = 0;
pub const DOMAIN_SMT_LEAF: u64 = 1;
pub const DOMAIN_NOTE_COMMITMENT: u64 = 2;
pub const DOMAIN_NULLIFIER: u64 = 3;
pub const DOMAIN_ADDRESS_SALT: u64 = 4;

// Raw Poseidon2 Permutation (t=4,t=8) wrapper around taceo-poseidon2 crate
fn permute_3(state: &[Fr;4]) -> [Fr;4] {
    taceo_poseidon2::bn254::t4::permutation(state)
}

// Domain - Seperated hash for fixed length inputs (3 inputs -> t4)

fn hash_3(domain: u64, a: Fr, b: Fr) -> Fr {
    let state = [Fr::from(domain), a,b,Fr::from(0u64)];
    permute_3(&state)[0]
}

// Domain Seperated hash for 5 fixed length inputs (5 inputs -> t8)
fn hash_5(domain: u64, a: Fr, b: Fr, c: Fr, d: Fr, e: Fr) -> Fr {
    let h1 = permute_3(&[Fr::from(domain), a, b, c])[0];
    let h2 = permute_3(&[h1, d, e, Fr::from(0u64)])[0];
    h2
}

// Public Apis
pub fn hash_merkle_node(left: Fr, right: Fr) -> Fr {
    hash_3(DOMAIN_MERKLE_NODE, left, right)
}

// SMT leaf
pub fn hash_smt_leaf(key: Fr, value: Fr) -> Fr {
    hash_3(DOMAIN_SMT_LEAF, key, value)
}

// Note Commitment
pub fn hash_note_commitment(sk: Fr, r: Fr, v: Fr, token: Fr, chain_id: Fr) -> Fr {
    hash_5(DOMAIN_NOTE_COMMITMENT, sk, r, v, token, chain_id)
}

// Nullifier
pub fn hash_nullifier(sk: Fr, rho: Fr) -> Fr {
    hash_3(DOMAIN_NULLIFIER, sk, rho)
}
// Address Salt
pub fn hash_address_salt(sk: Fr, salt: Fr) -> Fr {
    hash_3(DOMAIN_ADDRESS_SALT, sk, salt)
}


// Input Normalization 
// Error handling enum
pub enum InteropError {
    OutOfRange,
    InvalidInput,}

// convert 32 big-endian byltes to a field element. Rejects if >=p
pub fn bytes_to_field(bytes: &[u8; 32])->Result<Fr, InteropError> {
    let val = Fr::from_be_bytes_mod_order(bytes);
    let mut roundtrip =  [0u8; 32];
    val.into_bigint().to_bytes_be().iter().enumerate().for_each(|(i,&b)| {
        roundtrip[i] = b;
    });
    if roundtrip != *bytes {
        return Err(InteropError::OutOfRange);
    }
    Ok(val)
    
}
