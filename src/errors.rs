use alloy_sol_types::sol;
use stylus_sdk::prelude::*;

sol! {
    error BeforePubKeyRetrieval();
    error OutsideField();
    error Pairing();
    error PrimeVerification();
    error ScalarMulti();
    error Plus();
}

#[derive(SolidityError)]
pub enum VerifierError {
    PrimeVerification(PrimeVerification),
    OutsideField(OutsideField),
    ScalarMulti(ScalarMulti),
    Plus(Plus),
    Pairing(Pairing),
}
