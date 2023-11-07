// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use forust_ml::GradientBooster;
use methods::XGBOOST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use rmp_serde;
use serde_json;

const JSON_MODEL: &str = include_str!("../../res/trained_model.json");

fn main() {
    let result = predict();
    println!("Prediction from the trained XGBoost model is:  {}", &result);
}

fn predict() -> f64 {
    // We import the trained XGBoost model
    let xgboost: GradientBooster = serde_json::from_str(JSON_MODEL).unwrap();

    // We serialize the model to a byte array for transport to the guest
    let rmp_xgboost: Vec<u8> = rmp_serde::to_vec(&xgboost).unwrap();

    // We define an input value for the model (inputs are block number and numbe of transaction in that block.  Note we modify the block number to a f64 value).
    //**************************//
    // ADD YOUR INPUT DATA HERE //
    //**************************//
    let data: Vec<f64> = vec![18511304.0, 117.0];

    let env = ExecutorEnv::builder()
        .write(&data)
        .unwrap()
        .write(&rmp_xgboost)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, XGBOOST_ELF).unwrap();

    // We return the inference value comitted to the journal
    receipt.journal.decode().unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn basic() {
        const EXPECTED: f64 = 30.528042544062632;
        let result = super::preict();
        assert_eq!(EXPECTED, result);
    }
}