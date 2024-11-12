use winterfell::{
    crypto::hashers::Blake3_256,
    math::{fields::f128::BaseElement, FieldElement, ToElements},
    TraceTable
};

mod air;
pub use air::TrainAir;

mod prover;
pub use prover::DoWorkProver;

mod rescue;
use rescue::{CYCLE_LENGTH, NUM_ROUNDS};

pub mod utils;

pub type Blake3 = Blake3_256<BaseElement>;

pub const TRACE_WIDTH: usize = 4;

pub struct PublicInputs {
    pub seed: [BaseElement; 2],
    pub result: [BaseElement; 2],
}

impl ToElements<BaseElement> for PublicInputs {
    fn to_elements(&self) -> Vec<BaseElement> {
        let mut result = self.seed.to_vec();
        result.extend_from_slice(&self.result);
        result
    }
}

pub fn build_trace(seed: [BaseElement; 2]) -> TraceTable<BaseElement> {
    // Instantiate the trace with a given width and length; this will allocate all
    // required memory for the trace
    let trace_length = CYCLE_LENGTH;
        let mut trace = TraceTable::new(TRACE_WIDTH, trace_length);

        trace.fill(
            |state| {
                // initialize first state of the computation
                state[0] = seed[0];
                state[1] = seed[1];
                state[2] = BaseElement::ZERO;
                state[3] = BaseElement::ZERO;
            },
            |step, state| {
                // execute the transition function for all steps
                //
                // for the first 14 steps in every cycle, compute a single round of
                // Rescue hash; for the remaining 2 rounds, just carry over the values
                // in the first two registers to the next step
                if (step % CYCLE_LENGTH) < NUM_ROUNDS {
                    rescue::apply_round(state, step);
                } else {
                    state[2] = BaseElement::ZERO;
                    state[3] = BaseElement::ZERO;
                }
            },
        );
    trace
}



