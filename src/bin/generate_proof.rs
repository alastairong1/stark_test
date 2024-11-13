use std::fs::File;
use std::io::Write;
use std::time::Instant;
use winterfell::{
    math::{fields::f128::BaseElement, FieldElement}, FieldExtension, ProofOptions, Prover,
};
use stark_test::{build_trace, DoWorkProver};

fn main() {
    // Start timer
    let start_time = Instant::now();

    let seed = [BaseElement::from(42u8), BaseElement::from(43u8)];
    let n = 16;

    let trace = build_trace(seed);
    let result = [trace.get(0,n-1), trace.get(1, n-1)];

    

    let options = ProofOptions::new(
        32, // number of queries
        8,  // blowup factor
        0,  // grinding factor
        FieldExtension::None,
        8,   // FRI folding factor
        127, // FRI remainder max degree
    );

    let prover = DoWorkProver::new(options);

    // Write result and proof to files
    let result_path = "./artifacts/result.txt";
    let proof_path = "./artifacts/proof.txt";

    // Ensure the artifacts directory exists
    std::fs::create_dir_all("./artifacts").unwrap();

    // Write result to file
    let mut result_file = File::create(result_path).expect("Unable to create result file");
    let result_bytes = BaseElement::elements_as_bytes(&result);
    result_file
        .write_all(&result_bytes)
        .expect("Could not write result bytes to file");

    let proof = prover.prove(trace).unwrap();
    let proof_bytes = proof.to_bytes();
    // Write proof to file
    let mut proof_file = File::create(proof_path).expect("Unable to create proof file");
    proof_file
        .write_all(&proof_bytes)
        .expect("Could not write proof bytes to file");

    // Calculate elapsed time
    let elapsed = start_time.elapsed();

    // Print the time taken and file paths
    println!("Computation completed in: {:.2?}", elapsed);
    println!("Result written to: {}", result_path);
    println!("Proof written to: {}", proof_path);
}
