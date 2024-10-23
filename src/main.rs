use std::fs::File;
use std::io::Write;
use std::time::Instant;
use winterfell::math::fields::f128::BaseElement;
use winterfell::FieldExtension;
use winterfell::{ProofOptions, Prover};

fn main() {
    // Start timer
    let start_time = Instant::now();

    let start = BaseElement::new(3);
    let n = 1_048_576;

    let trace = stark_test::build_do_work_trace(start, n);
    let result = trace.get(0, n - 1);

    let options = ProofOptions::new(
        32, // number of queries
        8,  // blowup factor
        0,  // grinding factor
        FieldExtension::None,
        8,   // FRI folding factor
        127, // FRI remainder max degree
    );

    let prover = stark_test::DoWorkProver::new(options);

    // Write result and proof to files
    let result_path = "./artifacts/result.txt";
    let proof_path = "./artifacts/proof.txt";

    // Ensure the artifacts directory exists
    std::fs::create_dir_all("./artifacts").unwrap();

    // Write result to file
    let mut result_file = File::create(result_path).expect("Unable to create result file");
    writeln!(result_file, "{:?}", result).expect("Unable to write result to file");

    let proof = prover.prove(trace).unwrap();
    // Write proof to file
    let mut proof_file = File::create(proof_path).expect("Unable to create proof file");
    writeln!(proof_file, "{:?}", proof).expect("Unable to write proof to file");

    // Calculate elapsed time
    let elapsed = start_time.elapsed();

    // Print the time taken and file paths
    println!("Computation completed in: {:.2?}", elapsed);
    println!("Result written to: {}", result_path);
    println!("Proof written to: {}", proof_path);
}
