use std::env;
use std::fs;
use std::collections::HashMap;
fn main() {
    //red the file and  build vector of individual word
    let contents = match env::args().nth(1){
        Some(f) => match fs::read_to_string(f){
            Ok(s)=>s.to_lowercase(),
            Err(e)=>{
                eprint!("could not read the file {}",e);
                std::process::exit(1);
            }
        },
        None=>
        {
            eprint!("Program requires an argument : <file path>");
            std::process::exit(1);
        }
    };

}
