
use std::{collections::{HashSet, HashMap}, error::Error, fmt};
use Regex::*;

use crate::nfa::NFA;

pub const EPSILON_CHR: char = 127 as char;

#[derive(Debug, Clone)]
pub enum Regex {
    Epsilon,
    Char(char),
    Seqn( Box<Regex> , Box<Regex>),
    Or(Box<Regex> , Box<Regex>),
    Set(HashSet<char>),
    Star(Box<Regex>),
    Range(char,char),
    Plus(Box<Regex>),
    Str(&'static str),
    Optional(Box<Regex>),
    Name(&'static str)
}

#[derive(Debug)]
pub struct NameNotFoudError{
    message : String
}

impl fmt::Display for NameNotFoudError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}", self.message)
    }
}

impl Error for NameNotFoudError {}

pub type NamesList= HashMap<&'static str , Regex>;

pub type Delta = HashMap<(i32,char) , HashSet<i32>>;

pub fn expand_trans(delta : &mut Delta ,trans_in : (i32, char) , trans_out : HashSet<i32>) {

    if let Some(existing_trans_out) = delta.get_mut(&trans_in) {
        existing_trans_out.extend(trans_out.iter())
    }

    delta.insert(trans_in, trans_out);
}


impl Regex {


    pub fn from_str(string : &str) -> Self {
        let c = string.chars().nth(0);

        if c.is_some() {
            Seqn(Box::new(Char(c.unwrap())), Box::new(Self::from_str(&string[1..])))
        } else {
            Epsilon
        }
    }

    pub fn from_range(begin : char , end : char) -> Self {
        
        Set((begin..=end).collect())
    }


    pub fn all_except(chars_to_remove : HashSet<char>) -> Regex{

        Set(((0 as char)..=(126 as char)).filter(|c| !chars_to_remove.contains(c)).collect())

   }

    pub fn to_regular(&self, names : Option<&NamesList>) -> Result<Self , NameNotFoudError> {

        match self {

            Name(name) => {

                if names.is_none() {return Err(NameNotFoudError{message : "Regex contains a name but no name list was provided".to_string()});}

                if let Some(regex) = names.unwrap().get(name) {
                    regex.clone().to_regular(names)
                } else {
                    Err(NameNotFoudError{message : format!("Name not Found : {}", name)})
                }
            },

            Str(string) => {

                Ok(Regex::from_str(string))
            },

            Range(begin, end) => Ok(Regex::from_range(*begin, *end)),

            Plus(regex) => Ok(Seqn(Box::new(Self::to_regular(regex, names)?) , Box::new( Star(Box::new(regex.to_regular(names)?))))),            

            Optional(regex) => Ok(Or(Box::new(regex.to_regular(names)?), Box::new(Epsilon))),

            Or(regex1, regex2) => Ok(Or(Box::new(regex1.to_regular(names)?), Box::new(regex2.to_regular(names)?))),
            
            Seqn(regex1, regex2) => Ok(Seqn(Box::new(regex1.to_regular(names)?), Box::new(regex2.to_regular(names)?))),
            
            Star(regex) => Ok(Star(Box::new(regex.to_regular(names)?))),

            other => Ok(other.clone())
        }

    }


    fn create_nfa(&self, delta : &mut Delta, counter : &mut i32, current : i32) -> HashSet<i32>{

        match self {

            Epsilon => {
                let extremities = HashSet::from([*counter]);
                expand_trans(delta, (current , EPSILON_CHR), extremities.clone());
                *counter += 1;
                extremities
            },

            Char(c) => {
                
                let extremities = HashSet::from([*counter]);
                *counter += 1;
                expand_trans(delta,(current, *c) , extremities.clone());
                extremities
            },

            Set(set) => {

                let extremities = HashSet::from([*counter]);
                *counter += 1;
                for c in set {
                    expand_trans(delta,(current, *c) , extremities.clone());
                }
                extremities
                
            },

            Seqn(reg1, reg2) => {
                
                let left_out = reg1.create_nfa(delta, counter, current);

                let right_in = *counter;
                *counter += 1;

                let right_in_set =  HashSet::from([right_in]);

                for state in left_out {
                        
                    expand_trans(delta,(state, EPSILON_CHR) , right_in_set.clone());
                }

                reg2.create_nfa(delta, counter, right_in)
                
            }

            Or(reg1 , reg2) => {
                
                let left_in = *counter;
                *counter += 1;
                let left_out = reg1.create_nfa(delta, counter, left_in);
                
                let right_in = *counter;
                *counter += 1;

                let right_out = reg2.create_nfa(delta, counter, right_in);

                expand_trans(delta,(current, EPSILON_CHR) , HashSet::from([left_in , right_in]));
                
                let end = HashSet::from([*counter]);
                *counter += 1;
                
                for lstate in left_out {
                    expand_trans(delta,(lstate, EPSILON_CHR) , end.clone());
                }

                for rstate in right_out {
                    expand_trans(delta,(rstate, EPSILON_CHR) , end.clone());
                }
                
                end
            }

            Star(reg) => {
                
                let reg_in = *counter;
                *counter += 1;

                let reg_out = reg.create_nfa(delta, counter, reg_in);

                let state_after = *counter;
                *counter += 1;

                for state in reg_out {
                    expand_trans(delta, (state, EPSILON_CHR), HashSet::from([reg_in , state_after]));
                }


                expand_trans(delta, (current, EPSILON_CHR), HashSet::from([reg_in, state_after]));

                HashSet::from([state_after])
            }
            
            non_regular => non_regular.to_regular(None).unwrap().create_nfa(delta, counter, current)

        }

    } 


    pub fn to_nfa(&self, counter : &mut i32) -> NFA {

        let first = *counter;

        *counter += 1;

        let mut delta : Delta = HashMap::new();

        let finals = self.create_nfa(&mut delta, counter, first);

        NFA{delta : delta , initial_state : first , final_states : finals}
    }


}


#[macro_export]
macro_rules! Star {
    ($reg:expr) => {
        Star(Box::new($reg))
    };
}

#[macro_export]
macro_rules! Plus {
    ($reg:expr) => {
        Plus(Box::new($reg))
    };
}

#[macro_export]
macro_rules! Optional {
    ($reg:expr) => {
        Optional(Box::new($reg))
    };
}

#[macro_export]
macro_rules! Or {
    ($reg1:expr , $reg2:expr) => {
        Or(Box::new($reg1), Box::new($reg2))
    };
}

#[macro_export]
macro_rules! Seqn {
    ($reg1:expr , $reg2:expr) => {
        Seqn(Box::new($reg1), Box::new($reg2))
    };
}

#[macro_export]
macro_rules! Set {
    ($($v:expr),* $(,)?) => {
        Set(HashSet::from([$($v,)*]))
    };
}
