# Vectors

Test vector specification for the Mímir Poseidon2 Interop Kit.

## Fixture File Format

```json
{
  "name": "merkle_node_basic",
  "function": "hash_merkle_node",
  "domain": "MERKLE_NODE",
  "inputs": ["0x0000...0001", "0x0000...0002"],
  "expected_output": "0x..."
}
```

All field values are 0x-prefixed big-endian hex strings, zero-padded to 64 hex characters (32 bytes), per encoding.md Section 2.

## Required Test Categories

- Basic cases for each hash function (merkle node, smt leaf, nullifier, note commitment, address salt)
- Edge cases: zero inputs, max field element (p-1), single element
- Invalid cases: input >= p (should reject), null input (should reject)
- Barretenberg reference vector (raw permutation, no domain tag):
  - Input: [0, 1, 2, 3]
  - Output: [0x01bd538c2ee014ed5141b29e9ae240bf8db3fe5b9a38629a9647cf8d76c01737, 0x239b62e7db98aa3a2a8f6a0d2fa1709e7a35959aa6c7034814d9daa90cbac662, 0x04cbb44c61d928ed06808456bf758cbf0c18d1e15a7b6dbc8245fa7515d5e3cb, 0x2e11c5cff2a22c64d01304b778d78f6998eff1ab73163a35603f54794c30847a]

## Fixture Files

- `vectors/poseidon2_raw.json` - raw permutation vectors (no domain tag)
- `vectors/merkle_node.json` - domain-separated merkle node hashes
- `vectors/smt_leaf.json` - domain-separated SMT leaf hashes
- `vectors/note_commitment.json` - domain-separated note commitments
- `vectors/nullifier.json` - domain-separated nullifier hashes
- `vectors/address_salt.json` - domain-separated address salt hashes

## Generation Rules

- Vectors are generated from the Rust reference implementation only.
- All values serialized as big-endian hex strings with 0x prefix, zero-padded to 64 hex chars.
- Noir and Solidity implementations must produce identical outputs for all vectors.
- Fixture files are the single source of truth for cross-language conformance.
