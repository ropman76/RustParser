use std::convert::TryInto;

pub struct Lexer{
    input:Vec<char>,
    position:usize,
    readPosition:usize,
    ch: char

}

pub fn New(input_string:String) -> Lexer {
    Lexer  { 
            input: input_string.chars().collect(),
            position: 0,
            readPosition:0,
            ch:'\0'
    }
 
}

pub fn readChar(mut l:Lexer) -> Lexer {
    if l.readPosition >= l.input.len()
    {
        l.ch ='\0';
    } else {
        
     
      //  l.ch = l.input[l.readPosition];
        l.ch = l.input[0];
    }

    l.position = l.readPosition;
    l.readPosition += 1;
    l

}