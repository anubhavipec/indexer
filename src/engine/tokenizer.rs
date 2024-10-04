
use std::{io::Error, fs, io::Read};

pub fn tokenize(dir_path:&str) ->Result<Vec<String>,Error>{
    let mut token_list = Vec::new();
    for entry in fs::read_dir(dir_path)?{
        
        let entry = entry?;
        println!("checking path {:?}",entry);
        let path = entry.path();
        if path.is_file() {
            let mut file_handler = fs::File::open(path)?;
            let mut contents = String::new();
            file_handler.read_to_string(&mut contents)?;
            let tokens:Vec<String> = contents.split_whitespace()
            .map(|word| word.to_lowercase())
            .collect();
        token_list.extend(tokens);
        }
    }
    Ok(token_list)

}