# Poseidon2 Interop Kit

Domain-separated Poseidon2 hashing over BN254, with matching implementations in Rust, Noir, and Solidity (coming soon).

Built for the Mímir Collective privacy stack. Ensures that hashes computed off-chain (Rust) match those computed in-circuit (Noir) and on-chain (Solidity).

## Architecture

- **Permutation:** Poseidon2 with width t=4, 8 full rounds + 56 partial rounds, x^5 S-box
- **Domain separation:** First element of the permutation input is a domain tag (see `spec/domains.md`)
- **Multi-input chaining:** For >3 inputs, chain output[0] into position [0] of the next t=4 permutation
- **Output:** First element of post-permutation state (`output[0]`)
- **Field:** BN254 scalar field (p = 21888...5617)
- **Byte serialization:** 32-byte big-endian (see `spec/encoding.md`)

## Quick Start

### Rust

```toml
[dependencies]
mimir-poseidon2-interop = { git = "https://github.com/Mimir-Collective/poseidon2-interop-kit", path = "rust" }
```

```rust
use ark_bn254::Fr;
use mimir_poseidon2_interop::*;

let left = Fr::from(1u64);
let right = Fr::from(2u64);
let hash = hash_merkle_node(left, right);
```

### Noir

```toml
[dependencies]
mimir_poseidon2_interop = { git = "https://github.com/Mimir-Collective/poseidon2-interop-kit", directory = "noir" }
```

```rust
use mimir_poseidon2_interop::{hash_merkle_node, hash_nullifier};

let h = hash_merkle_node(left, right);
let n = hash_nullifier(sk, rho);
```

## Hash Functions

| Function | Domain | Inputs | Use Case |
|----------|--------|--------|----------|
| `hash_merkle_node(left, right)` | 0 | 2 | Internal Merkle tree nodes |
| `hash_smt_leaf(key, value)` | 1 | 2 | Sparse Merkle tree leaves |
| `hash_note_commitment(sk, r, v, token, chain_id)` | 2 | 5 | Shielded note commitments |
| `hash_nullifier(sk, rho)` | 3 | 2 | Spend nullifier derivation |
| `hash_address_salt(address, salt)` | 4 | 2 | Private address derivation |

## Running Tests

### Rust

```bash
cd rust
cargo test
```

### Noir

```bash
cd noir
nargo test
```

## Test Vectors

JSON fixture files in `vectors/` are the single source of truth for cross-language conformance. Generated from the Rust reference implementation.

To regenerate vectors:

```bash
cd rust
cargo test print_vectors -- --nocapture
```

Then update the JSON files in `vectors/` with the output.

## Specifications

- `spec/encoding.md` - Field encoding, byte serialization, Poseidon2 parameters
- `spec/domains.md` - Domain tag registry and rules
- `spec/vectors.md` - Test vector format and generation rules

## Project Status

- [x] Rust reference implementation
- [x] Noir implementation
- [x] Cross-language test vectors (JSON)
- [x] Barretenberg compatibility verified
- [ ] Solidity implementation
- [ ] CI pipeline
- [ ] npm/crate publishing

## License

MIT
