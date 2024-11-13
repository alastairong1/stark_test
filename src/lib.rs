use winterfell::{
    crypto::hashers::Blake3_256,
    math::{fields::f128::BaseElement, ExtensibleField, FieldElement, ToElements},
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

pub const TRACE_WIDTH: usize = 5;

pub struct PublicInputs {
    pub input_hash: [BaseElement; 2],
    pub output_hash: [BaseElement; 2],
}

impl ToElements<BaseElement> for PublicInputs {
    fn to_elements(&self) -> Vec<BaseElement> {
        let mut result = self.input_hash.to_vec();
        result.extend_from_slice(&self.output_hash);
        result
    }
}

    // There are 3 phases in this trace
    // 1. Hashing the seed to get input hash
    // 2. Multiplying the input hash x 2
    // 3. Hashing the input hash again to get output hash
    // The current approach hashes everything sequentially. When we start hashing two things together we should experiment
    // On hashing two parallel hashings
pub fn build_trace(seed: [BaseElement; 2]) -> TraceTable<BaseElement> {
    
    let trace_length = CYCLE_LENGTH;
        let mut trace = TraceTable::new(TRACE_WIDTH, trace_length);
        const TWO: BaseElement = BaseElement::new(2);

        trace.fill(
            |state| {
                // initialize first state of the computation
                state[0] = seed[0];
                state[1] = seed[1];
                state[2] = BaseElement::ZERO;
                state[3] = BaseElement::ZERO;
                state[4] = BaseElement::ZERO; // Phase
            },
            |step, state| {
                // execute the transition function for all steps
                //
                // for the first 14 steps in every cycle, compute a single round of
                // Rescue hash; for the remaining 2 rounds, just carry over the values
                // in the first two registers to the next step
                match state[4] {
                    BaseElement::ZERO => {
                        if (step % CYCLE_LENGTH) < NUM_ROUNDS {
                            rescue::apply_round(state, step);
                        } else {
                            state[2] = BaseElement::ZERO;
                            state[3] = BaseElement::ZERO;
                        }

                        // TODO: Add condition to increment phase
                    }

                    BaseElement::ONE => {
                        // TODO: Check that state[0] and state[1] are in fact the input hash that we want to multiply
                        // I think it is because it is the result returned from the basic hashing function
                        state[0] = state[0] * BaseElement::new(2);
                        state[1] = state[1] * BaseElement::new(2);
                        // state[2] = BaseElement::ZERO; // Reinitialise just in case. But this might be wasteful so remove in future?
                        // state[3] = BaseElement::ZERO; // Reinitialise just in case
                        state[4] = TWO; 
                    }

                    TWO => {
                        if (step % CYCLE_LENGTH) < NUM_ROUNDS { // TODO: This needs to get fixed due to the irregular phase size
                            rescue::apply_round(state, step);
                        } else {
                            state[2] = BaseElement::ZERO;
                            state[3] = BaseElement::ZERO;
                        }
                    }

                    _ => {
                        unreachable!("This phase shouldn't exist!")
                    }
                }
                
            },
        );
    trace
}



