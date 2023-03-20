use std::env;
use std::fs;
use std::collections::HashMap;
use std::vec;
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
        //count num of app this word 
        *word_count.entry(word).or_insert(0)+=1;

    }
    //detrmine the most commonly used  word
    let mut top_count =0u32;
    let mut top_words:Vec<&str> = Vec::new();
    for(&key,&val) in word_count.iter(){
        if val> top_count{
            top_count=val ;
            top_words.clear();
            top_words.push(key);
        }else if  top_count == val { 
            top_words.push(key);
            
        }
    }

}
