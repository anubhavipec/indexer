use std::{collections::{HashMap, HashSet}, io::Error};

use super::{model::Document, tokenizer::tokenize};


/*
    Changed our inverted Index data structure to HashMap<String,HashSet<String>> from HashMap<String,Vec<String>>,
    this will help us to do Binary Operations on search

 */
pub fn build_inverted_index(document_store:&Vec<Document>) -> Result<HashMap<String,HashSet<String>>,Error> {

    let mut index:HashMap<String,HashSet<String>> = HashMap::new();

    for doc in document_store {

        let tokens  = tokenize(&doc.text, " ");
        for token in tokens{
            index.entry(token).or_insert_with(HashSet::new).insert(doc.id.to_string());
        }
    }
    Ok(index)
}