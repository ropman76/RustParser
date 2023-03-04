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
         
            Litteral: String::new()  
        },
        ';' =>   Token {
            TokenType: token::SEMICOLON(),
          
            Litteral: String::new()  
           
        },
        '(' =>   Token {
            TokenType: token::LPAREN(),
           
            Litteral: String::new()  
           
        },
        ')' =>   Token {
            TokenType: token::RPAREN(),
          
            Litteral: String::new()  
          
        },
        ',' =>   Token {
            TokenType: token::COMMA(),
          
            Litteral: String::new()  
         
        },
        '+' =>   Token {
            TokenType: token::COMMA(),
          
            Litteral: String::new()  
           
        },
        '{' =>   Token {
            TokenType: token::LBRACE(),
           
            Litteral: String::new()  
            
        },
        '}' =>   Token {
            TokenType: token::RBRACE(),
          
            Litteral: String::new()  
        },
        '\0' =>   Token {
            TokenType: token::EOF(),
          
            Litteral: String::new()  
        },
        _ =>   Token {
            TokenType: token::EOF(),
          
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