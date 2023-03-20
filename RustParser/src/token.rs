use std::collections::HashMap;

const THRESHOLD: i32 = 10;
pub enum TokenType {
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

   
   
   pub struct Token{
       pub TokenType:TokenType,
       pub Litteral:String

    }
  

 pub const  ILLEGAL:String =  String::from("ILLEGAL");
   pub const  PLUS:String =  String::from("+");
   pub const EOF:String =  String::from("EOF");
   pub const IDENT:String =  String::from("IDENT");
   pub const INT:String =  String::from("INT");
   pub const ASSIGN:String =  String::from("=");
   pub const COMMA:String =  String::from(",");
   pub const SEMICOLON:String =  String::from(";");
   pub const LPAREN:String =  String::from("(");
   pub const RPAREN:String =  String::from(")");
   pub const LBRACE:String =  String::from("{");
   pub const RBRACE:String =  String::from("}");
   pub const FUNCTION:String =  String::from("FUNCTION");
   pub const LET:String =  String::from("LET");

   
   
  fn LookupIdentifierType(identifier : &str ) -> TokenType {
    let types = HashMap::from([
        ("fn", TokenType::FUNCTION),
        ("let", TokenType::LET),
        ("true",  TokenType::True),
        ("if", TokenType::IF),
        ("else", TokenType::Else),
        ("return", TokenType::Return),
        ]);

        let m = identifier;
        match types.get(m) {
        Some(tt ) => return tt ,
        _ => return  TokenType::ILLEGAL,
    }
  }
  


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
