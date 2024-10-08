use std::{collections::HashMap, env};

use engine::{indexer::build_inverted_index, model::Document, search::search, tokenizer::doc_splitter};

mod engine;

fn main() {

    let doc_storage:Vec<Document> = doc_splitter("src/book1.txt","CHAPTER")
        .unwrap_or_else(|err| {
        eprintln!("Error occured {}",err);
        Vec::new()
    });

    let index = build_inverted_index(doc_storage)
        .unwrap_or_else(|err|{
        eprintln!("Error Occured while building index {}",err);
        HashMap::new()
    });
    /*
    TODO: for next interation ==>  we can chain doc_splitter and build_inverted_index, 
     */
    let args:Vec<String> = env::args().collect();
    if args.len() > 1{
    let search_query_string = args.get(1).unwrap();
    let search_queries:Vec<&str> = search_query_string.split("AND").collect();

    // take a search query input as argument
    let result = search(args.get(1).unwrap(), &index);

    println!("Search Result {:?}",result);
    }
    else{
        println!("No search query provided");
    }

}
