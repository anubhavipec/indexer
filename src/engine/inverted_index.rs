use std::collections::HashMap;

use super::{model::Document, tokenizer::tokenize};


struct InvertedIndex {

    pub index: HashMap<String, HashMap<String,u16>>,
    pub doc_count: usize,
}


impl InvertedIndex{

    pub fn new() -> Self {

        InvertedIndex {
            index: HashMap::new(),
            doc_count: 0,
        }
    }

    pub fn add_documents(&mut self, document_store:&Vec<Document>) {

            for doc in document_store {

                let tokens = tokenize(&doc.text, " ");

                for token in tokens{

                    let token_entry = self.index.entry(token).or_insert(HashMap::new());
                    let counter = token_entry.entry(doc.id.to_string()).or_insert(0);
                    *counter += 1;
                }
                self.doc_count += 1;
            }
    }



}