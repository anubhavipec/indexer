use std::{collections::HashMap, hash::Hash, io::Error};

use super::{inverted_index::InvertedIndex, model::Document};
/*


*/



pub fn compute_idf(doc_count:usize,doc_freq: usize) -> f64 {
    (doc_count as f64 / (1.0 + doc_freq as f64)).ln()
}

pub fn compute_tfidf(tf:u64,idf:f64) -> f64 {
    (tf as f64) * idf
}

pub fn rank_documents(queries: Vec<String>, index:&InvertedIndex) -> HashMap<String,f64> {

    let mut doc_scores:HashMap<String,f64> = HashMap::new();

    for query in queries {
        if let Some(term_docs) = index.index.get(&query) {

            let idf = compute_idf(index.doc_count, term_docs.len());

            for (doc_id,tf) in term_docs {
                let tfidf = compute_tfidf(*tf as u64, idf);
                *doc_scores.entry(doc_id.clone()).or_insert(0.0) += tfidf;
                
            }
        }
    }
    doc_scores
}