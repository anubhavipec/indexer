use std::{collections::HashMap, io::Error};

use super::{model::Document, tokenizer::tokenize};



pub fn build_inverted_index(document_store:Vec<Document>) -> Result<HashMap<String,Vec<String>>,Error> {

    let mut index:HashMap<String,Vec<String>> = HashMap::new();

    for doc in document_store {

        let tokens  = tokenize(&doc.text, " ");
        for token in tokens{
            index.entry(token).or_insert_with(Vec::new).push(doc.id.to_string());
        }
    }
    Ok(index)
}