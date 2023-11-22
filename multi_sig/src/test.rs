#![cfg(test)]

use super::*;
extern crate std;
use base64::decode;

use arrayref::array_ref;
use serde_json::json;
use soroban_sdk::{symbol_short, Env};

//test::std::vec::Vec<u8>
fn base64_decode(encoded: &str) -> test::std::vec::Vec<u8> {
    let bytes = base64::decode(encoded).unwrap();
    return bytes;
}
fn convert_str_to_bytes(address: &str) -> test::std::vec::Vec<u8> {
    let bytes = base64::decode(address).unwrap();
    return bytes;
    // if let Ok(decoded_signature) = decode(address) {
    //     let validator_signature = decoded_signature;
    //      validator_signature
    //    // std::println!("validator_signature: {:?}", validator_signature);
    // } else {

    // }
}
fn convert_to_bytes32(address: &str) -> [u8; 32] {
    // Step 1: Decode the base64-encoded address into a byte array
    let decoded_address_bytes = base64_decode(address);

    // Step 2: Extract the public key portion of the address
    let public_key_bytes = &decoded_address_bytes[0..32];

    // Step 3: Pad the public key with zeros to make it a bytes32 array
    let mut bytes32_array = [0u8; 32];
    let length = public_key_bytes.len();
    bytes32_array[0..length].copy_from_slice(public_key_bytes);

    bytes32_array
}
#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MultiSignature);
    let client = MultiSignatureClient::new(&env, &contract_id);

    let validator1_signature = [
        86, 251, 119, 243, 119, 176, 162, 225, 26, 91, 249, 16, 243, 147, 195, 156, 249, 112, 72,
        232, 136, 28, 237, 195, 108, 119, 183, 80, 170, 153, 51, 79, 166, 106, 91, 188, 114, 14,
        100, 32, 27, 135, 187, 202, 109, 221, 110, 227, 167, 94, 170, 87, 183, 218, 78, 160, 85,
        109, 222, 214, 72, 37, 147, 8,
    ];

    let validator2_signature = [
        169, 188, 15, 43, 251, 46, 179, 44, 147, 167, 222, 235, 85, 35, 183, 117, 80, 122, 56, 218,
        153, 145, 161, 76, 0, 220, 71, 204, 156, 210, 221, 18, 185, 225, 33, 175, 155, 230, 51,
        247, 133, 128, 246, 29, 147, 39, 5, 117, 120, 46, 243, 122, 9, 91, 173, 58, 138, 32, 174,
        144, 224, 186, 53, 0,
    ];

    let validator3_signature = [
        167, 32, 108, 157, 55, 212, 116, 219, 11, 52, 172, 93, 95, 4, 180, 93, 188, 131, 161, 121,
        121, 211, 198, 73, 229, 130, 43, 77, 11, 225, 26, 6, 33, 234, 159, 49, 33, 210, 244, 234,
        29, 119, 201, 175, 54, 206, 201, 160, 172, 97, 106, 7, 211, 52, 75, 16, 53, 197, 0, 161,
        203, 139, 140, 15,
    ];

    let validator4_signature = [
        146, 44, 228, 15, 51, 242, 172, 19, 216, 9, 253, 231, 24, 207, 106, 88, 112, 144, 21, 136,
        208, 38, 80, 251, 234, 79, 86, 48, 215, 162, 68, 35, 138, 92, 109, 159, 81, 185, 215, 227,
        81, 116, 71, 205, 110, 123, 76, 167, 30, 231, 169, 141, 213, 144, 36, 202, 19, 21, 16, 97,
        48, 157, 98, 10,
    ];

    let validator5_signature = [
        128, 83, 64, 145, 5, 5, 129, 136, 181, 86, 125, 71, 36, 51, 140, 220, 217, 120, 255, 133,
        205, 100, 101, 168, 57, 206, 210, 43, 47, 136, 166, 83, 77, 45, 186, 54, 213, 114, 51, 190,
        219, 80, 195, 152, 188, 213, 111, 37, 96, 210, 96, 17, 86, 243, 118, 165, 23, 181, 200,
        215, 137, 105, 74, 7,
    ];
    let message = [
        34, 78, 101, 118, 101, 114, 32, 71, 105, 118, 101, 32, 85, 112, 34,
    ];
    let msg = message.into_val(&env);
    //1
    let validator_pubkey1 = "kVB8jPGZjChodmCQZkX9A3EGKjSFVMjEj4xINiNEzKA=";

    let public_key1 = convert_to_bytes32(validator_pubkey1);
    //  std::println!("public_key1 {:?}", public_key1);

    let pubkey1 = BytesN::from_array(&env, &public_key1);

    let signature1 = [validator1_signature];
    let sig1 = BytesN::from_array(&env, &signature1.get(0).unwrap());

    //2
    let validator_pubkey2 = "NpPp879xMEdH25kPxyJJ3Q8735w7o2O577NEYAPoidI=";
    let public_key2 = convert_to_bytes32(validator_pubkey2);
    // std::println!("public_key2 {:?}", public_key2);

    let pubkey2 = BytesN::from_array(&env, &public_key2);
    let signature2 = [validator2_signature];
    let sig2 = BytesN::from_array(&env, &signature2.get(0).unwrap());

    //3
    let validator_pubkey3 = "EnU2WsoNNCkRjqnQP+G9oR7rissxCmIBJRlJYimAA/Y=";
    let public_key3 = convert_to_bytes32(validator_pubkey3);
    let pubkey3 = BytesN::from_array(&env, &public_key3);
    // std::println!("public_key3 {:?}", public_key3);

    let signature3 = [validator3_signature];
    let sig3 = BytesN::from_array(&env, &signature3.get(0).unwrap());

    //4
    let validator_pubkey4 = "BfgU+gzKAZBL1ayZwAbqSyLqjnrl4ZCdSiWHC8Dv9AI=";
    let public_key4 = convert_to_bytes32(validator_pubkey4);
    //std::println!("public_key4 {:?}", public_key4);

    let pubkey4 = BytesN::from_array(&env, &public_key4);
    let signature4 = [validator4_signature];
    let sig4 = BytesN::from_array(&env, &signature4.get(0).unwrap());

    //5
    let validator_pubkey5 = "ykeNADs/kM+Ynw6FGuFMm0tILurmvR8ZeIuJ0yCnODs=";
    let public_key5 = convert_to_bytes32(validator_pubkey5);
    // std::println!("public_key5 {:?}", public_key5);

    let pubkey5 = BytesN::from_array(&env, &public_key5);
    let signature5 = [validator5_signature];
    let sig5 = BytesN::from_array(&env, &signature5.get(0).unwrap());

    let pub_vec = vec![&env, pubkey1, pubkey2, pubkey3, pubkey4, pubkey5];
    let sig_vec = vec![&env, sig1, sig2, sig3, sig4, sig5];
    client.claim(&pub_vec, &msg, &sig_vec);
}
