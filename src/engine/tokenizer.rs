
use std::{ fs, io::{Error, Read}};

use crate::engine::model::Document;

pub fn tokenize(dir_path:&str) ->Result<Vec<Document>,Error>{
    let mut token_list = Vec::new();
    let mut  document_store:Vec<Document> = Vec::new();
    for entry in fs::read_dir(dir_path)?{
        let entry = entry?;
        println!("checking path {:?}",entry);
        let path = entry.path();
        
        if path.is_file() {
            let mut file_handler = fs::File::open(path)?;
            let mut contents = String::new();
            file_handler.read_to_string(&mut contents)?;
            let tokens:Vec<String> = contents.split("CHAPTER")
            .map(|word| word.to_lowercase())
            .collect();

            token_list.extend(tokens);
        }
        
        for (chapter,text) in token_list.iter().enumerate(){
            let  document:Document = Document{id:format!{"Chapter {}",chapter+1},text:text.to_lowercase()};
            document_store.push(document);
        }
    }
    Ok(document_store)

}