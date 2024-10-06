use std::collections::HashMap;

use super::tokenizer::tokenize;



pub fn search<'a>(query: &'a str, index: &'a HashMap<String,Vec<String>>) -> Vec<&'a String> {

    let mut search_result = Vec::new();

    let query_tokens = tokenize(query," "); 

    for token in query_tokens{

        if let Some(doc) = index.get(&token) {
            search_result.extend(doc);
        }
    }

    search_result
}