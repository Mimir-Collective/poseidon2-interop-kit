# Encoding Specification

Canonical encoding rules for the Mímir Poseidon2 Interop Kit. All implementations must conform to this spec.

## Section 1 - Field Definition

- Field: BN254 scalar field
- Modulus: p = 21888242871839275222246405745257275088548364400416034343698204186575808495617
- All field elements are integers in the range [0, p-1]
- Element size: 32 bytes (256 bits)

## Section 2 - Byte Serialization

All field elements are serialized as 32-byte big-endian unsigned integers.

| Field Element | Big-Endian 32-byte Hex                                             |
| ------------- | ------------------------------------------------------------------ |
| 0             | 0x0000000000000000000000000000000000000000000000000000000000000000 |
| 1             | 0x0000000000000000000000000000000000000000000000000000000000000001 |
| 255           | 0x00000000000000000000000000000000000000000000000000000000000000ff |
| 256           | 0x0000000000000000000000000000000000000000000000000000000000000100 |
| 65536         | 0x0000000000000000000000000000000000000000000000000000000000010000 |

## Section 3 - Input Normalization

- If input >= field modulus p: REJECT. Do not silently reduce.
- Null or zero-length inputs: REJECT.
- Conversion from raw bytes to field element:
  1. Take input as 32 bytes
  2. Interpret as big-endian unsigned integer
  3. If integer >= p, reject. Otherwise, integer is the field element.
- Multi-element input ordering: see domains.md for per-function input sequences.

## Section 4 - Poseidon2 Parameters

| Parameter                 | Value                         |
| ------------------------- | ----------------------------- |
| Width (t)                 | 4                             |
| S-box degree (d)          | 5 (x^5)                       |
| Full rounds (rounds_f)    | 8 (4 before partial, 4 after) |
| Partial rounds (rounds_p) | 56                            |
| Total rounds              | 64                            |
| Field                     | BN254 scalar field            |
| sbox_size                 | 254 bits                      |

### Multi-Permutation Chaining (inputs > 3)

For hash functions requiring more than 3 inputs (domain tag + inputs > t=4 state):

- Split into multiple t=4 permutations
- First permutation: [domain_tag, input_0, input_1, input_2] → take output[0]
- Second permutation: [h1, input_3, input_4, 0] → take output[0]
- Pattern: chain output[0] into position [0] of next permutation

This applies to: hash_note_commitment (6 total elements = 2 permutations)

Noir's poseidon2_permutation only supports t=4. All implementations use t=4 chaining for consistency.

### Permutation Structure

```
initial_external_matrix → 4 full rounds → 56 partial rounds → 4 full rounds
```

- Full rounds: S-box applied to all 4 state elements
- Partial rounds: S-box applied to state[0] only

### External MDS Matrix

```
| 5  7  1  3 |
| 4  6  1  1 |
| 1  3  5  7 |
| 1  1  4  6 |
```

### Internal Matrix Diagonal (D_i - 1 form)

```
0x10dc6e9c006ea38b04b1e03b4bd9490c0d03f98929ca1d7fb56821fd19d3b6e7
0x0c28145b6a44df3e0149b3d0a30b3bb599df9756d4dd9b84a86b38cfb45a740b
0x00544b8338791518b2c7645a50392798b21f75bb60e3596170067d00141cac15
0x222c01175718386f2e2e82eb122789e352e105a3b8fa852613bc534433ee428b
```

### Round Constants

64 rounds x 4 field elements. Not reproduced here.

- Source file: `barretenberg/cpp/src/barretenberg/crypto/poseidon2/poseidon2_params.hpp`
- Generator: https://github.com/HorizenLabs/poseidon2/blob/main/poseidon2_rust_params.sage
- Author: Markus Schofnegger (Horizen Labs)

### Reference Test Vector (raw permutation)

- Input: [0, 1, 2, 3]
- Output: [0x01bd538c2ee014ed5141b29e9ae240bf8db3fe5b9a38629a9647cf8d76c01737, 0x239b62e7db98aa3a2a8f6a0d2fa1709e7a35959aa6c7034814d9daa90cbac662, 0x04cbb44c61d928ed06808456bf758cbf0c18d1e15a7b6dbc8245fa7515d5e3cb, 0x2e11c5cff2a22c64d01304b778d78f6998eff1ab73163a35603f54794c30847a]

## Section 5 - Domain Separation

- Domain tag is prepended as the first input element to the hash.
- Example: `hash_merkle_node(left, right)` computes `poseidon2(DOMAIN_MERKLE_NODE, left, right)`
- Tag values defined in domains.md.
- Tags are immutable after v1.

## Section 6 - In-Circuit Bit Decomposition

- In-circuit bit decomposition uses little-endian per Noir's `to_le_bits`.
- This applies ONLY inside circuits (comparisons, range checks).
- This does NOT affect byte serialization (which is big-endian per Section 2).
- Source: Noir stdlib convention, used in SMT `field_less_than` function.

## Section 7 - Output Format

- Output is a single BN254 field element (first element of post-permutation state).
- Serialized as a 32-byte big-endian array (same rule as inputs, per Section 2).
- No truncation.
- Range: always in [0, p-1].

### Example

- Input: [0, 1, 2, 3] (raw permutation)
- Output: `0x01bd538c2ee014ed5141b29e9ae240bf8db3fe5b9a38629a9647cf8d76c01737`
