# **rlex!** : ***Lex***icial Analiser Generator in ***R***ust 

Are you building your own programming language? Do you need to perform lexical analysis to arbritray language? 

Don't worry! *With rlex you can **r**e**lex**!*

This is a simple to use lexical analyser generator for Rust.

## Get Started 

``` rust

#[derive(Debug)]
enum ExpressionToken {
    Blank,
    Var(String),
    Val(i32)
    Plus,
    Times,
    Minus,
    Div
}

let lexer = LexerBuilder::from_names(HashMap::new([
    ("whitespace" , Set(HashSet::from([' ', '\n' , '\t' , '\r'])))
    ("letters", Star(Box::new(Or(Box::new(Range('a','z'))), Box::new(Range('A','Z')))))
    ("digits", Star(Box::new(Range('0','9'))))
]))
    .add_pattern(Name("whitespace"), |_x| ExpressionToken::Blank)
    .add_pattern(Name("letters"), |x| ExpressionToken::Var(x))
    .add_pattern(Name("digits"), |x| ExpressionToken::Val(x.parse::<i32>.unwrap()))
    .add_pattern(Name("digits"), |x| ExpressionToken::Val(x.parse::<i32>.unwrap()))
    .add_pattern(Char('+'), |_x| ExpressionToken::Plus)
    .add_pattern(Char('-'), |_x| ExpressionToken::Minus)
    .add_pattern(Char('*'), |_x| ExpressionToken::Times)
    .add_pattern(Str("rlex!", |_x| {println!("It feels good to rlex sometimes!") ; ExpressionToken::Empty}))
    .add_pattern(Char('/'), |_x| ExpressionToken::Div)

.build();

let text = "x + y * 2 relax"

for lexeme in lexer.lexemes(text) {
    
    println!("{:?}" ,lexeme.unwrap());

}

 ```


This will give the output : 
``` 
Var("x")
Blank
Plus
Blank
Var("y")
Blank
Times
Blank
Val(2)
It feels good to rlex sometimes!
Blank
```


## Projet contents

In the main.rs file you will find an implementation of a Python Lexer as an example of how to use rlex. I am still working on turning this project into a rust library and might upload it to crates.io!

## TODOs

Here are some things I would like to add to this project. Help would be appreciated if you want 😇.

- Add macros to make regular expressions easier to write
- Add a macro to build the lexer
- Consider changing the data structure for the transition function of the Lexer's internal NFA
- Build the Lexer at compile-time (or do the most amount of work at compile-time)
- Add memoisation for the epsilon-closure function