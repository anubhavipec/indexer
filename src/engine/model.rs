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