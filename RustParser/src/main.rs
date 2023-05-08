
use std::io;
use std::io::prelude::*;
mod token;
mod Lexer;


fn main() {
    println!("Welcome to the Money languauge.");
    println!("feel free  to typein in commands.");
    
    let mut Monkey_input = String::new();
   
   loop{
    io::stdin()
    .read_line(&mut Monkey_input)
    .expect("Failed to read line");

   }
  
    let nv =  Lexer::New(Monkey_input);

    let m =  token::LET;
    let z = token::LET;
    println!("{}",z);
}
