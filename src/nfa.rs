use crate::regex::{Delta, expand_trans};
use crate::regex::EPSILON_CHR;

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct NFA {

    pub delta : Delta,
    pub initial_state : i32,
    pub final_states : HashSet<i32>

}



impl NFA  {

    pub fn glue_nfas(nfa_vec : Vec<NFA>, new_fst_state : i32) -> Self {

        let mut new_delta : Delta = HashMap::new();

        let mut first_states : HashSet<i32> = HashSet::new(); 

        let mut final_states : HashSet<i32> = HashSet::new(); 
        
        for nfa in nfa_vec {

            first_states.insert(nfa.initial_state);
            final_states.extend(nfa.final_states.iter());

            for (trans_in , trans_out) in nfa.delta {

                expand_trans(&mut new_delta, trans_in, trans_out);
                
            }  

        }   

        expand_trans(&mut new_delta, (new_fst_state, EPSILON_CHR), first_states);

        NFA{delta : new_delta , initial_state : new_fst_state , final_states}

    }


    pub fn epsilon_closure(&self , states : HashSet<i32>) -> HashSet<i32>{

        let mut to_visit = states.clone();
        let mut visited : HashSet<i32>= HashSet::new();
        let mut result : HashSet<i32>= states.clone();

        while to_visit.len() != 0 {

            for state in to_visit.clone() {

                let reached_states = self.delta.get(&(state, EPSILON_CHR));

                if reached_states.is_some() {
                    result.extend(reached_states.unwrap().iter());
                    visited.insert(state);

                    for state in reached_states.unwrap() {
                        if !visited.contains(&state) {
                            to_visit.insert(*state);
                        }
                    }
                    
                }

  
                to_visit.remove(&state);
            }

        }
        
        result
        
    }

    pub fn make_transition(&self , current_states : HashSet<i32>, c : char) -> HashSet<i32> {

        let mut result : HashSet<i32> = HashSet::new();

        for state in current_states {

            if let Some(reached_states) = self.delta.get(&(state , c)) {
                result.extend(reached_states.iter());
            }   

        }

        result
    }

}

