use crate::token::Token;
use crate::token;






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
            TokenType: token::ASSIGN(),
            ToKenValue:l.ch.to_string(),
            Litteral: String::new()  
        },
        ';' =>   Token {
            TokenType: token::SEMICOLON(),
            ToKenValue:l.ch.to_string(),
            Litteral: String::new()  
           
        },
        '(' =>   Token {
            TokenType: token::LPAREN(),
            ToKenValue:l.ch.to_string(),
            Litteral: String::new()  
           
        },
        ')' =>   Token {
            TokenType: token::RPAREN(),
            ToKenValue:l.ch.to_string(),
            Litteral: String::new()  
          
        },
        ',' =>   Token {
            TokenType: token::COMMA(),
            ToKenValue:l.ch.to_string(),
            Litteral: String::new()  
         
        },
        '+' =>   Token {
            TokenType: token::COMMA(),
            ToKenValue:l.ch.to_string(),
            Litteral: String::new()  
           
        },
        '{' =>   Token {
            TokenType: token::LBRACE(),
            ToKenValue:l.ch.to_string(),
            Litteral: String::new()  
            
        },
        '}' =>   Token {
            TokenType: token::RBRACE(),
            ToKenValue:l.ch.to_string(),
            Litteral: String::new()  
        },
        '\0' =>   Token {
            TokenType: token::EOF(),
            ToKenValue:l.ch.to_string(),
            Litteral: String::new()  
        },
        _ =>   Token {
            TokenType: token::EOF(),
            ToKenValue:l.ch.to_string(),
            Litteral: String::new()  
        },

    };
    l = readChar(l);
    (tok,l)

}

fn IsLetter(c: char) -> bool {
    
    if c.is_alphabetic(){
        return true
    } else if c == '_'{
        return true
    } else {
        return false
    }


}