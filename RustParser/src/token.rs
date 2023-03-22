use std::collections::HashMap;



#[derive(Clone)]
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
     Return

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
       pub Litteral: String

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
  pub const LET:&str=  "LET";

   
   
  fn LookupIdentifierType(identifier : &str ) -> TokenType {
    let types = HashMap::from([
        ("fn",TokenType::FUNCTION),
        ("let", TokenType::LET),
        ("true",  TokenType::True),
        ("if", TokenType::IF),
        ("else", TokenType::Else),
        ("return", TokenType::Return),
        ]);

     
     let return_token:TokenType =  match types.get(identifier) {
       Some(T_Y) =>  T_Y.clone(),
        _ => return  TokenType::ILLEGAL,
    };
    return_token
  }
  


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn  ILLEGAL_test() {
        assert_eq!("ILLEGAL", TokenType::ILLEGAL);
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
