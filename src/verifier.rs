use crate::constants::{ConstantParams, Constants};
use crate::groth_16::{G1Point, G2Point, Groth16};
use alloy_primitives::U256;
use alloy_sol_types::sol;
use stylus_sdk::prelude::*;
sol_storage! {
    #[entrypoint]
    pub struct Verifier {}
}

sol! {
    struct VerifyingKey {
        G1Point alfa1;
        G2Point beta2;
        G2Point gamma2;
        G2Point delta2;
        G1Point[2] IC;
    }

    struct Proof {
        G1Point A;
        G2Point B;
        G1Point C;
    }
}

use crate::errors::*;

#[public]
impl Verifier {
    #[allow(non_snake_case)]
    pub fn verifyProof(proof: [U256; 8], input: U256) -> Result<bool, VerifierError> {
        let mut i = 0;
        while i < 8 {
            if proof[i] >= Constants.PRIME_Q() {
                return Err(VerifierError::PrimeVerification(PrimeVerification {}));
            }
            i += 1;
        }

        let proof = Proof {
            A: G1Point {
                X: proof[0],
                Y: proof[1],
            },
            B: G2Point {
                X: [proof[2], proof[3]],
                Y: [proof[4], proof[5]],
            },
            C: G1Point {
                X: proof[6],
                Y: proof[7],
            },
        };

        let verifying_key = Verifier::verifyingKey()?;

        // let vk_x = G1Point {
        //     X: U256::from(0),
        //     Y: U256::from(0),
        // };

        // WIP pairing call fails
        // let mut vk_x = Groth16::plus(&vk_x, &verifying_key.IC[0])?;
        if input >= Constants.SNARK_SCALAR_FIELD() {
            return Err(VerifierError::OutsideField(OutsideField {}));
        }
        let scalarmul = Groth16::scalar_mul(&verifying_key.IC[1], input)?;
        let vk_x = Groth16::plus(&verifying_key.IC[0], &scalarmul)?;

        // WIP
        #[allow(clippy::needless_range_loop)]
        // for z in 0..6 {
        //     if input[z] >= Constants.SNARK_SCALAR_FIELD() {
        //         return Err(VerifierError::OutsideField(OutsideField {}));
        //     }
        //     let scalarmul = Groth16::scalar_mul(&verifying_key.IC[z + 1], input[z])?;
        //     let val2 = Groth16::plus(&vk_x, &scalarmul)?;
        //     vk_x = val2;
        // }
        Groth16::pairing(
            Groth16::negate(proof.A),
            proof.B,
            verifying_key.alfa1,
            verifying_key.beta2,
            vk_x,
            verifying_key.gamma2,
            proof.C,
            verifying_key.delta2,
        )
    }
}

impl Verifier {
    #[allow(non_snake_case)]
    pub fn verifyingKey() -> Result<VerifyingKey, VerifierError> {
        let alfa1 = G1Point {
            X: U256::from_be_bytes([
                36, 174, 83, 100, 44, 41, 182, 239, 206, 121, 66, 8, 105, 198, 52, 13, 185, 177,
                247, 113, 47, 216, 102, 241, 187, 243, 89, 70, 48, 91, 38, 53,
            ]),
            Y: U256::from_be_bytes([
                18, 205, 145, 110, 230, 223, 59, 228, 123, 168, 7, 113, 210, 107, 84, 139, 182,
                203, 157, 130, 230, 32, 44, 85, 227, 55, 120, 251, 253, 102, 253, 207,
            ]),
        };
        let beta2 = G2Point {
            X: [
                U256::from_be_bytes([
                    22, 163, 250, 127, 59, 164, 182, 41, 2, 161, 7, 24, 80, 186, 102, 216, 54, 23,
                    48, 70, 188, 116, 184, 230, 214, 208, 2, 78, 197, 156, 100, 37,
                ]),
                U256::from_be_bytes([
                    14, 37, 201, 116, 92, 118, 77, 52, 202, 100, 55, 102, 42, 68, 210, 145, 207,
                    170, 11, 104, 249, 207, 195, 218, 29, 131, 16, 20, 17, 222, 146, 36,
                ]),
            ],
            Y: [
                U256::from_be_bytes([
                    43, 99, 184, 19, 236, 123, 207, 63, 20, 213, 47, 155, 170, 28, 162, 129, 168,
                    187, 7, 17, 182, 30, 248, 98, 151, 89, 85, 115, 36, 229, 142, 233,
                ]),
                U256::from_be_bytes([
                    31, 46, 89, 95, 244, 71, 222, 180, 14, 176, 93, 15, 178, 41, 24, 21, 49, 92,
                    88, 194, 194, 169, 239, 130, 57, 46, 67, 167, 201, 85, 252, 135,
                ]),
            ],
        };
        let gamma2 = G2Point {
            X: [
                U256::from_be_bytes([
                    25, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37, 241,
                    170, 73, 51, 53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194,
                ]),
                U256::from_be_bytes([
                    24, 0, 222, 239, 18, 31, 30, 118, 66, 106, 0, 102, 94, 92, 68, 121, 103, 67,
                    34, 212, 247, 94, 218, 221, 70, 222, 189, 92, 217, 146, 246, 237,
                ]),
            ],
            Y: [
                U256::from_be_bytes([
                    9, 6, 137, 208, 88, 95, 240, 117, 236, 158, 153, 173, 105, 12, 51, 149, 188,
                    75, 49, 51, 112, 179, 142, 243, 85, 172, 218, 220, 209, 34, 151, 91,
                ]),
                U256::from_be_bytes([
                    18, 200, 94, 165, 219, 140, 109, 235, 74, 171, 113, 128, 141, 203, 64, 143,
                    227, 209, 231, 105, 12, 67, 211, 123, 76, 230, 204, 1, 102, 250, 125, 170,
                ]),
            ],
        };
        let delta2 = G2Point {
            X: [
                U256::from_be_bytes([
                    38, 222, 50, 177, 5, 159, 144, 154, 138, 114, 191, 82, 89, 30, 98, 168, 66,
                    156, 246, 37, 60, 1, 8, 12, 10, 19, 242, 208, 169, 93, 189, 118,
                ]),
                U256::from_be_bytes([
                    0, 223, 10, 204, 180, 213, 56, 111, 41, 237, 244, 29, 118, 2, 166, 114, 33,
                    237, 97, 146, 60, 119, 86, 211, 240, 122, 56, 243, 213, 254, 68, 246,
                ]),
            ],
            Y: [
                U256::from_be_bytes([
                    15, 50, 109, 211, 88, 147, 226, 44, 79, 214, 33, 139, 102, 244, 201, 132, 12,
                    247, 25, 37, 91, 79, 121, 202, 125, 74, 150, 146, 123, 41, 1, 249,
                ]),
                U256::from_be_bytes([
                    45, 19, 228, 207, 53, 196, 163, 42, 76, 134, 185, 102, 221, 78, 117, 102, 21,
                    127, 92, 91, 20, 115, 43, 128, 10, 201, 172, 68, 143, 4, 46, 187,
                ]),
            ],
        };
        let ic = [
            G1Point {
                X: U256::from_be_bytes([
                    20, 213, 200, 151, 12, 53, 204, 24, 110, 220, 25, 91, 18, 168, 59, 191, 255,
                    159, 172, 220, 95, 17, 49, 140, 143, 51, 159, 21, 23, 14, 78, 79,
                ]),
                Y: U256::from_be_bytes([
                    41, 188, 252, 201, 58, 93, 4, 118, 51, 218, 237, 15, 245, 52, 213, 83, 237,
                    130, 220, 67, 139, 5, 48, 14, 252, 117, 94, 27, 238, 43, 179, 214,
                ]),
            },
            G1Point {
                X: U256::from_be_bytes([
                    13, 96, 20, 130, 74, 118, 120, 204, 244, 46, 119, 252, 49, 136, 201, 183, 244,
                    160, 235, 204, 95, 227, 173, 79, 224, 157, 133, 129, 56, 46, 66, 95,
                ]),
                Y: U256::from_be_bytes([
                    27, 254, 16, 223, 137, 76, 205, 54, 169, 173, 168, 190, 152, 173, 90, 163, 217,
                    5, 125, 89, 110, 155, 222, 58, 153, 187, 3, 18, 236, 176, 160, 195,
                ]),
            },
        ];
        Ok(VerifyingKey {
            alfa1,
            beta2,
            gamma2,
            delta2,
            IC: ic,
        })
    }
}
