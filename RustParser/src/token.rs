



#[derive(Clone,Copy)]
pub enum  TokenType {
     ILLEGAL, 
     EOF,
     IDENT,
     INT,
     ASSIGN,
     PLUS,
     COMMA,
     SEMICOLON,
     LPAREN,
     RPAREN,
     LBRACE,
     RBRACE,
     FUNCTION,
     LET,
     True,
     IF,
     Else,
     Return,
     EQ,
     Not_EQ


}
     /*
   impl Default for TokenType{
    fn default() -> Self {
      TokenType{
            ILLEGAL: String::from("ILLEGAL"),
            EOF: String::from("EOF"),
            IDENT: String::from("IDENT"),
            INT:String::from("INT"),
              ASSIGN:String::from("="),
              PLUS:String::from("+"),
              COMMA:String::from(","),
              SEMICOLON:String::from(";"),
              LPAREN:String::from("("),
              RPAREN:String::from(")"),
              LBRACE:String::from("{"),
              RBRACE:String::from("}"),
              FUNCTION:String::from("FUNCTION"),
              LET:String::from("let"),
              True:String::from("true"),
              IF:String::from("If"),
              Else:String::from("else"),
              Return:String::from("return")
            }
        }
      
    }
   */
   
   pub struct Token{
       pub TokenType:TokenType,
       pub Litteral:String

    }
  impl Token {
   pub fn new(n:char) -> Token{
        let tok =  match n {
            '=' =>   Token {
                TokenType: TokenType::ASSIGN,
             
                Litteral: ASSIGN.to_string()
            },
            ';' =>   Token {
                TokenType: TokenType::SEMICOLON,
              
                Litteral: SEMICOLON.to_string()
               
            },
            '(' =>   Token {
                TokenType: TokenType::LPAREN,
               
                Litteral: LPAREN.to_string()
               
            },
            ')' =>   Token {
                TokenType: TokenType::RPAREN,
              
                Litteral: RPAREN.to_string()
              
            },
            ',' =>   Token {
                TokenType: TokenType::COMMA,
              
                Litteral: COMMA.to_string()
             
            },
            '+' =>   Token {
                TokenType: TokenType::PLUS,
              
                Litteral: PLUS.to_string()
               
            },
            '{' =>   Token {
                TokenType: TokenType::LBRACE,
               
                Litteral: LBRACE.to_string()
                
            },
            '}' =>   Token {
                TokenType: TokenType::RBRACE,
              
                Litteral: RBRACE.to_string()  
            },
            '\0' =>   Token {
                TokenType: TokenType::EOF,
              
                Litteral: EOF.to_string()  
            },
            _ =>   Token {
                TokenType: TokenType::EOF,
              
                Litteral: EOF.to_string()
            },
    
        };
        tok
    }
  }

   pub const  ILLEGAL:&str =  "ILLEGAL";
   pub const PLUS:&str =  "+";
   pub const EOF:&str =  "EOF";
   pub const IDENT:&str =  "IDENT";
   pub const INT:&str=  "INT";
   pub const ASSIGN:&str=  "=";
   pub const COMMA:&str =  ",";
   pub const SEMICOLON:&str = ";";
   pub const LPAREN:&str =  "(";
   pub const RPAREN:&str =  ")";
   pub const LBRACE:&str =  "{";
   pub const RBRACE:&str =  "}";
   pub const FUNCTION:&str =  "FUNCTION";
   pub const LET:&str= "LET";
   pub const EQ:&str=  "=="; 
   pub const Not_EQ:&str=  "!=";

   
   
 
  


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn  ILLEGAL_test() {
        assert_eq!("ILLEGAL", ILLEGAL);
    }

    #[test]
    fn  EOF_test() {
        assert_eq!("EOF", EOF);
    }
    #[test]
    fn  IDENT_test() {
        assert_eq!("IDENT", IDENT);
    }

    #[test]
    fn  INT_test() {
        assert_eq!("INT", INT);
    }

    #[test]
    fn  ASSIGN_test() {
        assert_eq!("=", ASSIGN);
    }

    #[test]
    fn  PLUS_test() {
        assert_eq!("+", PLUS);
    }

    #[test]
    fn  COMMA_test() {
        assert_eq!(",", COMMA);
    }
    #[test]
    fn  SEMICOLON_test() {
        assert_eq!(";", SEMICOLON);
    }

    #[test]
    fn  LPAREN_test() {
        assert_eq!("(", LPAREN);
    }

    #[test]
    fn  RPAREN_test() {
        assert_eq!(")", RPAREN);
    }


    #[test]
    fn  LBRACE_test() {
        assert_eq!("{", LBRACE);
    }

    #[test]
    fn  RBRACE_test() {
        assert_eq!("}", RBRACE);
    }

    #[test]
    fn  FUNCTION_test() {
        assert_eq!("FUNCTION", FUNCTION);
    }
    fn  LET_test() {
        assert_eq!("LET", LET);
    }
}
