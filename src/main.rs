use std::env;
use std::fs;
use std::collections::HashMap;
fn main() {
    //red the file and  build vector of individual word
    let contents = match env::args().nth(1){
        //read the  file to string 
        Some(f) => match fs::read_to_string(f){
            //make all text  lower case 
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
    //lets gett all thz word make the itor to a  vector 
    let all_words =contents.split_whitespace().collect::<Vec<&str>>();
    //count  how many time each uniq words occors
    let mut word_count:HashMap<&str,u32> =HashMap::new();
    for word in all_words.iter(){
        *word_count.entry(word).or_insert(0)+=1;

    }
    //detrmine the most commonly used  word

}
