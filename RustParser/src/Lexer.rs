pub struct Lexer{
    input:String,
    position:i32,
    readPosition:i32,
    ch:i32

}

pub fn New(input:String) -> Lexer {
    let l = Lexer{ input: String::from(input) };
    l
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