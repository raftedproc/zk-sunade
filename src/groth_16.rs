use alloy_primitives::Address;
use alloy_sol_types::sol;
use stylus_sdk::{alloy_primitives::U256, call::RawCall, prelude::*};

use crate::constants::{ConstantParams, Constants};
use crate::errors::*;

sol_storage! {
    pub struct Groth16 {}
}

sol! {
    struct G1Point {
        uint256 X;
        uint256 Y;
    }

    // Encoding of field elements is: X[0] * z + X[1]
    struct G2Point {
        uint256[2] X;
        uint256[2] Y;
    }
}

impl Groth16 {
    pub fn negate(p: G1Point) -> G1Point {
        if p.X == U256::ZERO && p.Y == U256::ZERO {
            G1Point {
                X: U256::ZERO,
                Y: U256::ZERO,
            }
        } else {
            G1Point {
                X: p.X,
                Y: Constants.PRIME_Q() - (p.Y % Constants.PRIME_Q()),
            }
        }
    }

    pub fn plus(p1: &G1Point, p2: &G1Point) -> Result<G1Point, VerifierError> {
        let calldata = [p1.X, p1.Y, p2.X, p2.Y]
            .map(|i| i.to_be_bytes::<32>())
            .concat();
        let call_result = RawCall::new_static().gas(u64::MAX).call(
            Address::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6]),
            &calldata,
        );
        if call_result.is_err() {
            return Err(VerifierError::Plus(Plus {}));
        }
        let returndata = call_result.unwrap();
        Ok(G1Point {
            X: U256::from_be_bytes::<32>(returndata[0..32].try_into().unwrap()),
            Y: U256::from_be_bytes::<32>(returndata[32..64].try_into().unwrap()),
        })
    }

    pub fn scalar_mul(p1: &G1Point, s: U256) -> Result<G1Point, VerifierError> {
        let calldata = [p1.X, p1.Y, s].map(|i| i.to_be_bytes::<32>()).concat();
        // let calldata = ;
        let call_result = RawCall::new_static().gas(u64::MAX).call(
            Address::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7]),
            &calldata,
        );

        if call_result.is_err() {
            return Err(VerifierError::ScalarMulti(ScalarMulti {}));
        }

        let returndata = call_result.unwrap();
        Ok(G1Point {
            X: U256::from_be_bytes::<32>(returndata[0..32].try_into().unwrap()),
            Y: U256::from_be_bytes::<32>(returndata[32..64].try_into().unwrap()),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn pairing(
        a1: G1Point,
        a2: G2Point,
        b1: G1Point,
        b2: G2Point,
        c1: G1Point,
        c2: G2Point,
        d1: G1Point,
        d2: G2Point,
    ) -> Result<bool, VerifierError> {
        let p1 = [a1, b1, c1, d1];
        let p2 = [a2, b2, c2, d2];

        let mut input = [U256::ZERO; 24];

        for i in 0..4 {
            let j = i * 6;
            input[j] = p1[i].X;
            input[j + 1] = p1[i].Y;
            input[j + 2] = p2[i].X[0];
            input[j + 3] = p2[i].X[1];
            input[j + 4] = p2[i].Y[0];
            input[j + 5] = p2[i].Y[1];
        }

        let calldata = input.map(|i| i.to_be_bytes::<32>()).concat();
        let call_result = RawCall::new_static().gas(u64::MAX).call(
            Address::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8]),
            &calldata,
        );

        if call_result.is_err() {
            return Err(VerifierError::Pairing(Pairing {}));
        }
        let returndata = call_result.unwrap();
        let len = U256::from_be_bytes::<32>(returndata[0..32].try_into().unwrap());
        Ok(len != U256::from(0))
    }
}
