use std::{collections::HashMap, env, ops};

use engine::{indexer::build_inverted_index, model::{Document, Ops, QueryOperations, TermFrequency}, query_processing::{self, parse_search_queries}, ranking::calculate_term_frequency, search::search, tokenizer::doc_splitter};

mod engine;

fn main() {

    let doc_storage:Vec<Document> = doc_splitter("src/book1.txt","CHAPTER")
        .unwrap_or_else(|err| {
        eprintln!("Error occured {}",err);
        Vec::new()
    });

    let index = build_inverted_index(&doc_storage)
        .unwrap_or_else(|err|{
        eprintln!("Error Occured while building index {}",err);
        HashMap::new()
    });
    /*
    TODO: for next interation ==>  we can chain doc_splitter and build_inverted_index, 
     */
    let mut  tf_map = HashMap::new();
    for document in &doc_storage{
        
        tf_map.insert(document.id.to_string(), calculate_term_frequency(&document).unwrap());
        
    }
    let num_of_docs = &doc_storage.len();
    let tf = TermFrequency{token_fq_map:tf_map,total_docs:num_of_docs.clone()};
    
    
    let args:Vec<String> = env::args().collect();
    if args.len() > 1{
    let search_query_string = args.get(1).unwrap();
    let search_queries:QueryOperations = parse_search_queries(search_query_string).unwrap_or_else(|err| {
        eprintln!("Error occured while parsing search query {}" ,err);
        QueryOperations{op:Ops::DEFAULT,queries:Vec::new()}
    });

    println!("query parsed to {:?}",search_queries);

    // take a search query input as argument
    let result = search(&search_queries, &index);

    println!("Search Result {:?}",result);
    }
    else{
        println!("No search query provided");
    }

}
