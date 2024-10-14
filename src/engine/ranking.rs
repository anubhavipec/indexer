use std::{collections::HashMap, io::Error};

use super::model::Document;
/*


*/

pub fn calculate_term_frequency(document:&Document) -> Result<HashMap<String,u16>,Error>{

    let mut term_fq_map:HashMap<String, u16> = HashMap::new();
        for word in document.text.split_whitespace().into_iter() {
            *term_fq_map.entry(word.to_string()).or_insert(0) += 1;
    }
    Ok(term_fq_map)
}