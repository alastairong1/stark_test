use std::fs::File;
use std::io::Read;
use std::time::Instant;
use winterfell::math::fields::f128::BaseElement;
use winterfell::{Deserializable, Proof};

use winterfell::{
    crypto::DefaultRandomCoin,
    verify, AcceptableOptions
};

use stark_test::{PublicInputs, TrainAir, Blake3};

fn main() {
    // Start timer
    let start_time = Instant::now();

    let start = BaseElement::new(3);

    let result_path = "./artifacts/result.txt";
    let proof_path = "./artifacts/proof.txt";

    // Read the result from result.txt
    let mut result_file = File::open(result_path).expect("Failed to open result file");
    let mut result_bytes = Vec::new();
    result_file
        .read_to_end(&mut result_bytes)
        .expect("Could not read results");
    let result = BaseElement::read_from_bytes(&result_bytes).unwrap();

    // Read the proof from proof.txt
    let mut proof_file = File::open(proof_path).expect("Failed to open proof file");
    let mut proof_bytes = Vec::new();
    proof_file
        .read_to_end(&mut proof_bytes)
        .expect("Could not read proof");
    let proof = Proof::from_bytes(&proof_bytes).unwrap();

    let verify_result = verify_do_work(start, result, proof);
    println!("Verification outcome: {:?}", verify_result);
    // Calculate elapsed time
    let elapsed = start_time.elapsed();

    // Print the time taken and file paths
    println!("Computation completed in: {:.2?}", elapsed);
}


pub fn verify_do_work(seed: [BaseElement; 2], result: [BaseElement; 2], proof: Proof) {
    // The verifier will accept proofs with parameters which guarantee 95 bits or more of
    // conjectured security
    let min_opts = AcceptableOptions::MinConjecturedSecurity(95);

    // The number of steps and options are encoded in the proof itself, so we don't need to
    // pass them explicitly to the verifier.
    let pub_inputs = PublicInputs { seed, result };
    match verify::<TrainAir, Blake3, DefaultRandomCoin<Blake3>>(proof, pub_inputs, &min_opts) {
        Ok(_) => println!("yay! all good!"),
        Err(_) => panic!("something went terribly wrong!"),
    }
}
