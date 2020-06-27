pub struct Lexer{
    input:String,
    position:i32,
    readPosition:i32,
    ch:i32

}

pub fn new_lexer (input_string:String) -> Lexer {
    Lexer {
        input:String::from(input_string),
        position,
        readPosition,
        ch
    }


}

pub fn New(input_string:String) -> Lexer {
        input: String::from(input_string),
        position = 0,
        readPosition = 0,
        chi = 0

 
}

pub fn readChar(l:Lexer) -> Lexer {
    if(l.readPosition >= len(l.input))
    {
        l.ch =0;
    } else {
        l.ch = l.input[l.readPosition]
    }

    l.position = l.readPosition;
    l.readPosition += 1;
    l

}