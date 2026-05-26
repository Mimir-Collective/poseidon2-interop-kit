use ark_bn254::Fr;
use ark_ff::PrimeField;

#[test]
fn test_taceo_matches_barretenberg() {
    let state = [Fr::from(0u64), Fr::from(1u64), Fr::from(2u64), Fr::from(3u64)];
    let result = taceo_poseidon2::bn254::t4::permutation(&state);

    let expected = [
        Fr::from_be_bytes_mod_order(&hex::decode("01bd538c2ee014ed5141b29e9ae240bf8db3fe5b9a38629a9647cf8d76c01737").unwrap()),
        Fr::from_be_bytes_mod_order(&hex::decode("239b62e7db98aa3a2a8f6a0d2fa1709e7a35959aa6c7034814d9daa90cbac662").unwrap()),
        Fr::from_be_bytes_mod_order(&hex::decode("04cbb44c61d928ed06808456bf758cbf0c18d1e15a7b6dbc8245fa7515d5e3cb").unwrap()),
        Fr::from_be_bytes_mod_order(&hex::decode("2e11c5cff2a22c64d01304b778d78f6998eff1ab73163a35603f54794c30847a").unwrap()),
    ];

    assert_eq!(result[0], expected[0],"Result 0 Mismatch");
    assert_eq!(result[1], expected[1],"Result 1 Mismatch");
    assert_eq!(result[2], expected[2],"Result 2 Mismatch");
    assert_eq!(result[3], expected[3],"Result 3 Mismatch");


}