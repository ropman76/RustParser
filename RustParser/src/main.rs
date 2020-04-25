
 mod Token;
 use Token::ILLEGAL;
 mod Lexer;
fn main() {
   

  let m =  Token::ILLEGAL();
  let z = Token::LET();
  println!("{}",z);
}
