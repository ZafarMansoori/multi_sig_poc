#![no_std]
use soroban_sdk::{
    contract, contractimpl, symbol_short, vec, Bytes, BytesN, Env, IntoVal, Symbol, Vec,
};
extern crate std;
#[contract]
pub struct MultiSignature;
//fn compare_validator_public_key(env: Env, key_to_compare: BytesN<32>) -> bool {
fn compare_validator_public_key(env: Env, key_to_compare: BytesN<32>) -> bool {
    let mut result: Bytes = key_to_compare.into_val(&env);

    let VALIDATOR1: Bytes = [
        145, 80, 124, 140, 241, 153, 140, 40, 104, 118, 96, 144, 102, 69, 253, 3, 113, 6, 42, 52,
        133, 84, 200, 196, 143, 140, 72, 54, 35, 68, 204, 160,
    ]
    .into_val(&env);

    let VALIDATOR2: Bytes = [
        54, 147, 233, 243, 191, 113, 48, 71, 71, 219, 153, 15, 199, 34, 73, 221, 15, 59, 223, 156,
        59, 163, 99, 185, 239, 179, 68, 96, 3, 232, 137, 210,
    ]
    .into_val(&env);

    let VALIDATOR3: Bytes = [
        18, 117, 54, 90, 202, 13, 52, 41, 17, 142, 169, 208, 63, 225, 189, 161, 30, 235, 138, 203,
        49, 10, 98, 1, 37, 25, 73, 98, 41, 128, 3, 246,
    ]
    .into_val(&env);

    let VALIDATOR4: Bytes = [
        5, 248, 20, 250, 12, 202, 1, 144, 75, 213, 172, 153, 192, 6, 234, 75, 34, 234, 142, 122,
        229, 225, 144, 157, 74, 37, 135, 11, 192, 239, 244, 2,
    ]
    .into_val(&env);

    let VALIDATOR5: Bytes = [
        202, 71, 141, 0, 59, 63, 144, 207, 152, 159, 14, 133, 26, 225, 76, 155, 75, 72, 46, 234,
        230, 189, 31, 25, 120, 139, 137, 211, 32, 167, 56, 59,
    ]
    .into_val(&env);

    // let mut result: Bytes = key_to_compare.into_val(&env);

    let CURRENT_VALIDATOR: [Bytes; 5] =
        [VALIDATOR1, VALIDATOR2, VALIDATOR3, VALIDATOR4, VALIDATOR5];

    let mut IS_TRUE: bool = false;

    for i in 0..CURRENT_VALIDATOR.len() {
        //  std::println!("{:?}", CURRENT_VALIDATOR[i]);

        if CURRENT_VALIDATOR[i] == result {
            IS_TRUE = true;
            break;
        } else {
            IS_TRUE = false
        }
    }
    if IS_TRUE == true {
        return true;
    } else {
        return false;
    }
}
//
#[contractimpl]
impl MultiSignature {
    pub fn claim(
        env: Env,
        public_key: Vec<BytesN<32>>,
        message: Bytes,
        signature: Vec<BytesN<64>>,
    ) -> u32 {
        // std::println!("{:?}", public_key);

        let length = public_key.len();

        for i in 0..public_key.len() {
            let pubkey = public_key.get(i).unwrap();

            let sig = signature.get(i).unwrap();

            let is_true = compare_validator_public_key(env.clone(), pubkey.clone());

            if is_true == true {
                env.crypto().ed25519_verify(&pubkey, &message, &sig);
                std::println!("verification done : {:?}", i)
            } else {
                std::println!("Not a valid public key : {:?}", i)
            }
        }
        length
    }
}

mod test;
