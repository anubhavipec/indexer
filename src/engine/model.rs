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
pub struct InvertedIndex {

    pub index: HashMap<String,HashMap<String,u16>>, // this relation whill hold token -> (document_id , frequency of that token in respective document_id)
    pub total_docs: usize,

}