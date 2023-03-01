
mod regex;
mod nfa;
mod lexer;

use regex::Regex::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use lexer::*;
use std::fs;
use std::env;

#[derive(Debug, PartialEq)]
pub enum PythonToken {
    
    Empty, // for skipping
    
    // Keywords
    For, In, While, Del, If, Else, Elif, With, Import, From, As, Assert, Break, Continue, Class, Def, Except,
    False, True, Not, And, Or, None, Finally, Global, Is, Lambda, Try, Return, Yield, Pass, Raise, NonLocal,
    
    // Operators
    Eq, Eq2, Geq, Greater, Less, Leq, Neq, Plus, Minus, Star, Star2, Div, Mod, IntDiv, RShift, LShift, Caret,
    PlusEq, MinusEq, StarEq, Star2Eq, DivEq, ModEq, IntDivEq, CaretEq, LShitEq, RShiftEq, AmpersandEq, PipeEq,
    Wallrus, Ampersand,
    
    // idk how to classify those
    Bang,
    Tilde,
    
    // Separators
    Colon, SemiColon, Comma, BSlash,  Dot, OP, CP, OB, CB, CC, OC,
    Diamond,
    Slash2,

    // Literals
    IntLiteral(i32),
    FloatLiteral(f32),
    StrLiteral(String),
    
    // Identifier
    Identifier(String),
    Comment(String)

} 


fn main() {


    let lex = LexerBuilder::from_names(
        HashMap::from([
            ("whitespace", Set!{' ','\t','\n', '\r'}),
            ("lower", Range('a','z')),
            ("upper", Range('A','Z')),
            ("letter", Or!(Name("lower") ,Name("upper"))),
            ("letters", Plus!(Name("letter"))),
            ("digit", Range('0','9')),
            ("digits", Star!(Name("digit"))),
            ("letters_digits_symbols", Range(33 as char , 126 as char)),
            ("any", Range(0 as char , 126 as char)),
            ("letter_or_underscore", Or!(Name("letter"),Char('_'))),
            ("valid_identifier", Seqn!(Name("letter_or_underscore"),Star!(Or!(Name("digit"), Name("letter_or_underscore")))))
        ])
    )
        .add_pattern(Name("whitespace"), |_x| PythonToken::Empty)

        .add_pattern(Str("for"), |_x| PythonToken::For)
        .add_pattern(Str("in"), |_x| PythonToken::In)
        .add_pattern(Str("while"), |_x| PythonToken::While)
        .add_pattern(Str("del"), |_x| PythonToken::Del)
        .add_pattern(Str("if"), |_x| PythonToken::If)
        .add_pattern(Str("else"), |_x| PythonToken::Else)
        .add_pattern(Str("elif"), |_x| PythonToken::Elif)
        .add_pattern(Str("with"), |_x| PythonToken::With)
        .add_pattern(Str("import"), |_x| PythonToken::Import)
        .add_pattern(Str("from"), |_x| PythonToken::From)
        .add_pattern(Str("as"), |_x| PythonToken::As)
        .add_pattern(Str("assert"), |_x| PythonToken::Assert)
        .add_pattern(Str("break"), |_x| PythonToken::Break)
        .add_pattern(Str("continue"), |_x| PythonToken::Continue)
        .add_pattern(Str("class"), |_x| PythonToken::Class)
        .add_pattern(Str("def"), |_x| PythonToken::Def)
        .add_pattern(Str("e_xcept"), |_x| PythonToken::Except)
        .add_pattern(Str("False"), |_x| PythonToken::False)
        .add_pattern(Str("True"), |_x| PythonToken::True)
        .add_pattern(Str("not"), |_x| PythonToken::Not)
        .add_pattern(Str("and"), |_x| PythonToken::And)
        .add_pattern(Str("or"), |_x| PythonToken::Or)
        .add_pattern(Str("None"), |_x| PythonToken::None)
        .add_pattern(Str("finally"), |_x| PythonToken::Finally)
        .add_pattern(Str("global"), |_x| PythonToken::Global)
        .add_pattern(Str("is"), |_x| PythonToken::Is)
        .add_pattern(Str("lambda"), |_x| PythonToken::Lambda)
        .add_pattern(Str("try"), |_x| PythonToken::Try)
        .add_pattern(Str("return"), |_x| PythonToken::Return)
        .add_pattern(Str("yield"), |_x| PythonToken::Yield)
        .add_pattern(Str("pass"), |_x| PythonToken::Pass)
        .add_pattern(Str("raise"), |_x| PythonToken::Raise)
        .add_pattern(Str("nonlocal"), |_x| PythonToken::NonLocal)
        .add_pattern(Char('='), |_x| PythonToken::Eq)
        .add_pattern(Str("=="), |_x| PythonToken::Eq2)
        .add_pattern(Str(">="), |_x| PythonToken::Geq)
        .add_pattern(Char('>'), |_x| PythonToken::Greater)
        .add_pattern(Char('<'), |_x| PythonToken::Less)
        .add_pattern(Str("<="), |_x| PythonToken::Leq)
        .add_pattern(Str("!="), |_x| PythonToken::Neq)
        .add_pattern(Char('+'), |_x| PythonToken::Plus)
        .add_pattern(Char('-'), |_x| PythonToken::Minus)
        .add_pattern(Char('*'), |_x| PythonToken::Star)
        .add_pattern(Str("**"), |_x| PythonToken::Star2)
        .add_pattern(Char('/'), |_x| PythonToken::Div)
        .add_pattern(Char('%'), |_x| PythonToken::Mod)
        .add_pattern(Str("//"), |_x| PythonToken::IntDiv)
        .add_pattern(Str(">>"), |_x| PythonToken::RShift)
        .add_pattern(Str("<<"), |_x| PythonToken::LShift)
        .add_pattern(Char('^'), |_x| PythonToken::Caret)
        .add_pattern(Str("+="), |_x| PythonToken::PlusEq)
        .add_pattern(Str("-="), |_x| PythonToken::MinusEq)
        .add_pattern(Str("*="), |_x| PythonToken::StarEq)
        .add_pattern(Str("**="), |_x| PythonToken::Star2Eq)
        .add_pattern(Str("/="), |_x| PythonToken::DivEq)
        .add_pattern(Str("%="), |_x| PythonToken::ModEq)
        .add_pattern(Str("//="), |_x| PythonToken::IntDivEq)
        .add_pattern(Str("^="), |_x| PythonToken::CaretEq)
        .add_pattern(Str("<<="), |_x| PythonToken::LShitEq)
        .add_pattern(Str(">>="), |_x| PythonToken::RShiftEq)
        .add_pattern(Str("&="), |_x| PythonToken::AmpersandEq)
        .add_pattern(Str("|="), |_x| PythonToken::PipeEq)
        .add_pattern(Str(":="), |_x| PythonToken::Wallrus)
        .add_pattern(Char(':'), |_x| PythonToken::Colon)
        .add_pattern(Char(';'), |_x| PythonToken::SemiColon)
        .add_pattern(Char(','), |_x| PythonToken::Comma)
        .add_pattern(Char('\\'), |_x| PythonToken::BSlash) 
        .add_pattern(Char('.'), |_x| PythonToken::Dot)
        .add_pattern(Char('('), |_x| PythonToken::OP)
        .add_pattern(Char(')'), |_x| PythonToken::CP)
        .add_pattern(Char('['), |_x| PythonToken::OB)
        .add_pattern(Char(']'), |_x| PythonToken::CB)
        .add_pattern(Char('{'), |_x| PythonToken::CC)
        .add_pattern(Char('}'), |_x| PythonToken::OC)
        .add_pattern(Char('!'), |_x| PythonToken::Bang)
        .add_pattern(Char('~'), |_x| PythonToken::Tilde)
        .add_pattern(Char('&'), |_x| PythonToken::Ampersand)
        .add_pattern(Str("<>"), |_x| PythonToken::Diamond)
        .add_pattern(Name("digits"), |x| PythonToken::IntLiteral(x.parse::<i32>().unwrap()))
        .add_pattern(Seqn!(Star!(Name("digit")), Seqn!(Char('.'), Star!(Name("digit")))), |x| PythonToken::FloatLiteral(x.parse::<f32>().unwrap()))
        .add_pattern(Seqn!(Char('"'), Seqn!(Star!(Regex::all_except(HashSet::from(['"']))), Char('"'))),|x| PythonToken::StrLiteral(x))
        .add_pattern(Seqn!(Char('\''), Seqn!( Star!(Regex::all_except(HashSet::from(['\'']))), Char('\''))),|x| PythonToken::StrLiteral(x))
        .add_pattern(Seqn!(Str("'''"), Seqn!(Star!(Name("any")), Str("'''"))),|x| PythonToken::StrLiteral(x))
        .add_pattern(Seqn!(Str("\"\"\""), Seqn!(Star!(Name("any")), Str("\"\"\""))),|x| PythonToken::StrLiteral(x))
        .add_pattern(Name("valid_identifier"), |x| PythonToken::Identifier(x)) 
        .add_pattern(Seqn!(Char('#'), Seqn!( Star!(Regex::all_except(HashSet::from(['\n','\r']))), Set!{'\n','\r'})), |x| PythonToken::Comment(x))

    .build();


    for file_path in env::args().skip(1) {
        
        let binding = fs::read_to_string(file_path.clone())
        .expect(format!("Could not open file {}" , file_path).as_str());
        
        let text = binding.as_str();    
        
        println!("File contents : \n {}", text);
        
        
        lex.lexemes(text)
        .filter(|x| {
            if let Ok(tok) = x {
                    *tok != PythonToken::Empty
                }else {
                    true
                }
            })
            .for_each(|x| println!("{:?}", x));
    }


}
