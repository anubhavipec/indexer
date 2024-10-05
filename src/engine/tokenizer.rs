
use std::{ fs, io::{Error, Read}};

use crate::engine::model::Document;

pub fn doc_splitter(dir_path:&str,split_string:&str) ->Result<Vec<Document>,Error>{
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
            let tokens:Vec<String> = tokenize(contents.as_str(), split_string);
            token_list.extend(tokens);
        }
        
        for (id,text) in token_list.iter().enumerate(){
            let  document:Document = Document{id:format!{"{} {}",split_string,id+1},text:text.to_lowercase()};
            document_store.push(document);
        }
    }
    Ok(document_store)

}

pub fn tokenize(content:&str,split_string: &str) -> Vec<String> {
    content.split(split_string)
            .map(|word| word.to_lowercase())
            .collect()
}