use std::collections::HashMap;


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
  


    pub fn ILLEGAL() -> String
    {
        "ILLEGAL".to_string()
    }
    
    pub fn EOF() -> String
    {
        "EOF".to_string()
    }

    pub fn IDENT() -> String
    {
        "IDENT".to_string()
    }

    pub fn INT() -> String
    {
        "INT".to_string()
    }

    pub fn ASSIGN() -> String
    {
        "=".to_string()
    }

    pub fn PLUS() -> String
    {
        "+".to_string()
    }


    pub fn COMMA() -> String
    {
        ",".to_string()
    }

    pub fn SEMICOLON() -> String
    {
        ";".to_string()
    }
    pub fn LPAREN() -> String
    {
        "(".to_string()
    }
    pub fn RPAREN() -> String
    {
        ")".to_string()
    }
    pub fn LBRACE() -> String
    {
        "{".to_string()
    }
    pub fn RBRACE() -> String
    {
        "}".to_string()
    }
    pub fn FUNCTION() -> String
    {
        "FUNCTION".to_string()
    }
    pub fn LET() -> String
    {
        "LET".to_string()
    }
   
  fn LookupIdentifierType(identifier : &str ) -> TokenType {
    let solar_distance = HashMap::from([
        ("fn", TokenType::FUNCTION),
        ("let", TokenType::LET),
        ("true",  TokenType::True),
        ("if", TokenType::IF),
        ("else", TokenType::Else),
        ("return", TokenType::Return),
    ]);


  }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn  ILLEGAL_test() {
        assert_eq!("ILLEGAL", ILLEGAL());
    }

    #[test]
    fn  EOF_test() {
        assert_eq!("EOF", EOF());
    }
    #[test]
    fn  IDENT_test() {
        assert_eq!("IDENT", IDENT());
    }

    #[test]
    fn  INT_test() {
        assert_eq!("INT", INT());
    }

    #[test]
    fn  ASSIGN_test() {
        assert_eq!("=", ASSIGN());
    }

    #[test]
    fn  PLUS_test() {
        assert_eq!("+", PLUS());
    }

    #[test]
    fn  COMMA_test() {
        assert_eq!(",", COMMA());
    }
    #[test]
    fn  SEMICOLON_test() {
        assert_eq!(";", SEMICOLON());
    }

    #[test]
    fn  LPAREN_test() {
        assert_eq!("(", LPAREN());
    }

    #[test]
    fn  RPAREN_test() {
        assert_eq!(")", RPAREN());
    }


    #[test]
    fn  LBRACE_test() {
        assert_eq!("{", LBRACE());
    }

    #[test]
    fn  RBRACE_test() {
        assert_eq!("}", RBRACE());
    }

    #[test]
    fn  FUNCTION_test() {
        assert_eq!("FUNCTION", FUNCTION());
    }
    fn  LET_test() {
        assert_eq!("LET", LET());
    }
}
