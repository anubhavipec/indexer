use engine::{indexer::build_inverted_index, model::Document, tokenizer::doc_splitter};

mod engine;

fn main() {
    println!("Hello, world!");

    let doc_storage:Vec<Document> = doc_splitter("/Users/anubhav/Documents/rust-test/","CHAPTER")
    .unwrap_or_else(|err| {
        eprintln!("Error occured {}",err);
        Vec::new()
    });

    let index = build_inverted_index(doc_storage);


}
