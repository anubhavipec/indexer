pub struct Document {
    pub id: String,
    pub text:String,
}

pub enum Ops{
    AND,
    OR,
    DEFAULT,
}

pub struct QueryOperations<'a> {

    pub op: Ops,
    pub queries: Vec<&'a str>,
}