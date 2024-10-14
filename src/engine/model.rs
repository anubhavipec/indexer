use std::collections::HashMap;

pub struct Document {
    pub id: String,
    pub text:String,
}


/*
for comparing enums with  == we need to use macro derive(PartialEq,Eq)
 */
#[derive(PartialEq,Eq,Debug)]
pub enum Ops{
    AND,
    OR,
    DEFAULT,
}

#[derive(Debug)]
pub struct QueryOperations {

    pub op: Ops,
    pub queries: Vec<String>,
}


#[derive(Debug)]
pub struct TermFrequency{

    pub token_fq_map: HashMap<String,HashMap<String,u16>>, // this relation whill hold documentId -> (token  , frequency of that token in respective document_id)
    pub total_docs: usize,

}