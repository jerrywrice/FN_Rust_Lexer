use lexical_scanner;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0usize {
        eprintln!("Missing file path command argument. Aborting.");
        return;
    }
       
    let _token_list = lexical_scanner::lexer(&args[1]) ; 
    for (i, token) in _token_list.iter().enumerate(){
       eprintln!("{}. {:?}", i, token);
    }
}
