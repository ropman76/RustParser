
   pub struct Token{
        TokenType:String,
        ToKenValue:String

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
   
