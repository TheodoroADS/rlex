
use crate::nfa::{NFA};
use crate::regex::{NamesList, Regex, self};
use std::collections::{LinkedList, HashSet, HashMap};
use std::error::Error;
use std::{fmt};

pub struct Lexer<T> {

    pub nfa : NFA,
    bindings : HashMap<i32, fn(String) -> T>
}

pub struct LexerBuilder<T> {

    names : NamesList,
    patterns : LinkedList<(Regex , fn(String) -> T)>

}

pub struct LexemeStream<'a , T> {

    lexer : &'a Lexer<T>,
    text_chars : Vec<char>,
    next_tok_start_idx : usize,
}


#[derive(Debug)]
pub struct LexemeNotRecognisedErr{
    message : String
}

impl fmt::Display for LexemeNotRecognisedErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}", self.message)
    }
}


impl <T> LexerBuilder<T> {

    pub fn new(abbreviations : NamesList, patterns : LinkedList<(Regex, fn(String) -> T)>) -> Self {
        
        LexerBuilder { names: abbreviations, patterns: patterns }
        
    }
    
    pub fn from_names(abbreviations : NamesList) -> Self {
        
        LexerBuilder { names: abbreviations, patterns: LinkedList::new() }
    }

    pub fn add_pattern(mut self , reg : Regex , func : fn(String) -> T ) -> Self {

        self.patterns.push_back((reg, func));

        self
    }

    pub fn build(self) -> Lexer<T> {
        Lexer::new(self.names , self.patterns)
    }

}


impl<T> Lexer<T> {


    pub fn new(abbreviations : NamesList, patterns : LinkedList<(Regex, fn(String) -> T)>) -> Self {

        let mut counter = 0;
        let mut nfa_vec : Vec<NFA> = Vec::new();

        let mut bindings : HashMap<i32, fn(String) -> T> = HashMap::new();

        for (pattern, func ) in patterns { 

            let nfa = pattern.to_regular(Some(&abbreviations)).unwrap().to_nfa(&mut counter);
            
            for state in &nfa.final_states {

                bindings.insert(*state, func);
            }

            nfa_vec.push(nfa);

        }

        let glued = NFA::glue_nfas(nfa_vec, counter);

        Lexer{nfa : glued , bindings : bindings}
    }


    pub fn lexemes(&self, text : &str) -> LexemeStream<T> {

        let chars : Vec<char> = text.chars().collect(); 

        LexemeStream { lexer: &self, text_chars: chars, next_tok_start_idx: 0 }
    }


} 


impl Error for LexemeNotRecognisedErr {}

impl<'a , T> LexemeStream<'a , T> {


    pub fn get_next_token(&mut self) -> Option<Result<T, LexemeNotRecognisedErr>> {
        
        let mut stack : LinkedList<(usize,usize,HashSet<i32>)> = LinkedList::new();
        let mut current_char_idx = self.next_tok_start_idx;

        if current_char_idx >= self.text_chars.len() - 1 {
            return None
        }  

        let mut current_states = self.lexer.nfa.epsilon_closure(HashSet::from([self.lexer.nfa.initial_state]));
        

        loop {
            
            
            if current_char_idx == self.text_chars.len(){
                break;
            }
            
            let current_char = self.text_chars[current_char_idx];
            
            let reached_states =   
            self.lexer.nfa.epsilon_closure(
                self.lexer.nfa.make_transition(
                    current_states.clone(), 
                    current_char));
                    
            current_states = reached_states.clone();

            let is_empty = reached_states.is_empty();

            stack.push_front((self.next_tok_start_idx, current_char_idx, reached_states));
            
            if is_empty {
                break;
            }


            current_char_idx += 1;
        }

        for (tok_start_idx, tok_end_idx, states ) in stack {

            if let Some(fstate) = states.intersection(&self.lexer.nfa.final_states).min().take() {

                let tok_str : String = self.text_chars[tok_start_idx..=tok_end_idx].into_iter().collect();
                self.next_tok_start_idx = tok_end_idx + 1;
                return Some(Ok(self.lexer.bindings.get(fstate).unwrap()(tok_str)));

            }

        }

        let err = Some(Err(LexemeNotRecognisedErr { message: format!("Unkown token at position {}",
             self.next_tok_start_idx)}));
        self.next_tok_start_idx += 1;

        err
    }



}


impl <'a, T> Iterator for LexemeStream<'a , T> {

    type Item = Result<T, LexemeNotRecognisedErr>;

    fn next(&mut self) -> Option<Self::Item> {

        self.get_next_token()

    }

}

