pub struct Document {
    pub id: String,
    pub text:String,
}

#[derive(PartialEq,Eq)]
pub enum Ops{
    AND,
    OR,
    DEFAULT,
}

pub struct QueryOperations {

    pub op: Ops,
    pub queries: Vec<String>,
}