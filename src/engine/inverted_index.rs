use std::collections::{HashMap, HashSet};

use super::{model::Document, tokenizer::tokenize};


pub struct InvertedIndex {

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

    pub fn search_with_default_ops(&self, query:&str)-> HashSet<String>{

            self.index.get(query)
            .map(|doc_map| doc_map.keys().cloned().collect())
            .unwrap_or_else(HashSet::new)
    }

    pub fn search_with_and_ops(&self, queries:Vec<String>) -> HashSet<String> {
        let mut result = HashSet::new();

        for (i,query) in queries.iter().enumerate() {

            let docs = self.search_with_default_ops(query);
            if i == 0 {
                result = docs;
            }else {
                result = result.intersection(&docs).cloned().collect();
            }
        }
        result
    }

    pub fn search_with_or_ops(&self, queries:Vec<String>) -> HashSet<String> {
        let mut result = HashSet::new();

        for (i,query) in queries.iter().enumerate() {

            let docs = self.search_with_default_ops(query);
            if i == 0 {
                result = docs;
            }else {
                result = result.union(&docs).cloned().collect();
            }
        }
        result
    }





}