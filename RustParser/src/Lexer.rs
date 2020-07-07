pub struct Lexer{
    input:Vec<char>,
    position:i32,
    readPosition:i32,
    ch:i32

}

pub fn New(input_string:String) -> Lexer {
    Lexer  { 
            input: input_string.chars().collect(),
            position: 0,
            readPosition:0,
            ch:0
    }
 
}

pub fn readChar(l:Lexer) -> Lexer {
    if(l.readPosition >= len(l.input))
    {
        l.ch =0;
    } else {
        l.ch = l.input[l.readPosition];
       
    }

    l.position = l.readPosition;
    l.readPosition += 1;
    l

}