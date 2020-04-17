pub mod  Token {
   pub struct Token{
        TokenType:String,
        ToKenValue:String

    }

    pub const ILLEGAL:String = "ILLEGAL";
    pub const EOF:String = "EOF";
    pub const IDENT:String = "IDENT";
    pub const INT:String = "INT";
    pub const ASSIGN:String = "=";
    pub const PLUS:String = "+";
    
    pub const COMMA:String = ",";
    pub const SEMICOLON:String = ";";
   
    pub const LPAREN:String = "(";
    pub const RPAREN:String = ")";
    pub const LBRACE:String = "{";
    pub const RBRACE:String = "}";
    pub const FUNCTION:String = "FUNCTION";
    pub const LET:String = "LET";

}