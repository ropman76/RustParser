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


pub fn NextToken(mut l:Lexer)  -> (Token,Lexer){

 //  let tok =  Token;

  let tok =  match l.ch {
        '=' =>   Token {
            TokenType: MainToken::ASSIGN(),
            ToKenValue:l.ch.to_string()
        },
        ';' =>   Token {
            TokenType: MainToken::SEMICOLON(),
            ToKenValue:l.ch.to_string()
        },
        '(' =>   Token {
            TokenType: MainToken::LPAREN(),
            ToKenValue:l.ch.to_string()
        },
        ')' =>   Token {
            TokenType: MainToken::RPAREN(),
            ToKenValue:l.ch.to_string()
        },
        ',' =>   Token {
            TokenType: MainToken::COMMA(),
            ToKenValue:l.ch.to_string()
        },
        '+' =>   Token {
            TokenType: MainToken::COMMA(),
            ToKenValue:l.ch.to_string()
        },
        '{' =>   Token {
            TokenType: MainToken::LBRACE(),
            ToKenValue:l.ch.to_string()
        },
        '}' =>   Token {
            TokenType: MainToken::RBRACE(),
            ToKenValue:l.ch.to_string()
        },
        '\0' =>   Token {
            TokenType: MainToken::EOF(),
            ToKenValue:l.ch.to_string()
        },
        _ =>   Token {
            TokenType: MainToken::EOF(),
            ToKenValue:l.ch.to_string()
        },

    };
    l = readChar(l);
    (tok,l)

}