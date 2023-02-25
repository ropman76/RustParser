use crate::MainToken::Token;
use crate::MainToken;






pub struct Lexer{
    input:Vec<char>,
    position:usize,
    readPosition:usize,
    ch: char

}

pub fn New(input_string:String) -> Lexer {
  let l =  Lexer  { 
            input: input_string.chars().collect(),
            position: 0,
            readPosition:0,
            ch:'\0'
    };
    let m = readChar(l);
    m
}

pub fn readChar(mut l:Lexer) -> Lexer {
    if l.readPosition >= l.input.len()
    {
        l.ch ='\0';
    } else {
        
     
       l.ch = l.input[l.readPosition];
       // l.ch = l.input[0];
    }

    l.position = l.readPosition;
    l.readPosition += 1;
    l

}


pub fn NextToken(mut l:Lexer)  -> Token{

   let tok =  Token;

    match l.ch {
        '=' =>   tok = Token {
            TokenType: MainToken::ASSIGN(),
            ToKenValue:l.ch
        },
         
    }

    tok

}