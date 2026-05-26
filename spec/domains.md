# Domain Separation Registry

Domain tags for the Mímir Poseidon2 Interop Kit. Each tag is prepended as the first input element to the hash function to prevent cross-type collisions.

## Tag Registry

| Tag Name        | Value | Inputs (after tag)        | Description                    |
| --------------- | ----- | ------------------------- | ------------------------------ |
| MERKLE_NODE     | 0     | left, right               | Internal Merkle tree node hash |
| SMT_LEAF        | 1     | key, value                | Sparse Merkle tree leaf entry  |
| NOTE_COMMITMENT | 2     | sk, r, v, token, chain_id | Shielded note commitment       |
| NULLIFIER       | 3     | sk, rho                   | Spend nullifier derivation     |
| ADDRESS_SALT    | 4     | address, salt             | Private address derivation     |

## Hash Function Signatures

```
hash_merkle_node(left, right)         → poseidon2(0, left, right)
hash_smt_leaf(key, value)             → poseidon2(1, key, value)
hash_note_commitment(sk, r, v, token, chain_id) → poseidon2(2, sk, r, v, token, chain_id)
hash_nullifier(sk, rho)               → poseidon2(3, sk, rho)
hash_address_salt(address, salt)      → poseidon2(4, address, salt)
```

## Rules

1. No two object types may share a tag value.
2. Tags are immutable after v1. Once assigned, a value is permanently bound to its object type.
3. New tags are added by incrementing from the highest existing value.
4. Retired tags are never reused.
5. Domain tag is always the FIRST element in the input array.

## Migration Notes

- The existing SMT (`noir-sparse-merkle`) uses Poseidon v1 via `poseidon::poseidon::bn254::hash_2` with NO domain separation.
- Migration to Poseidon2 with domain-separated hashing is planned after interop kit Phase 2 (Rust + Noir adapters) is complete.
- Migration is a BREAKING CHANGE: all tree roots will differ. Test fixtures must be regenerated from the Rust reference implementation.
- New code (nullifiers, note commitments, compliance circuits) should use domain-separated Poseidon2 wrappers from day one.
