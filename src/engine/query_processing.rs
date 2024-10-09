use std::io::Error;

use super::model::QueryOperations;
use super::model::Ops;

const  AND_OPERATOR :&str = "AND";
const  OR_OPERATOR :&str = "OR";


/*
The reason the compiler asked you to add lifetime annotations is because you're working with references (&str) in the struct. Lifetimes are required in Rust when references are involved, as the compiler needs to know how long those references will be valid.

Here's a step-by-step breakdown of why lifetimes are needed and why the error occurred:

1. Original Struct with References:
When you originally defined:

rust
Copy code
pub struct QueryOperations<'a> {
    pub op: Ops,
    pub queries: Vec<&'a str>,  // References with a lifetime `'a`
}
Vec<&'a str> holds references to strings, so the lifetime 'a specifies how long those references must be valid.
This lifetime 'a needs to apply both to the struct itself and to the function that returns it.
2. Function's Return Type:
If you're returning a reference to the QueryOperations struct, your function will also need lifetime annotations to ensure that all references within the struct live long enough. For example:

rust
Copy code
pub fn parse_search_queries<'a>(search_query: &'a str) -> Result<&'a QueryOperations<'a>, Error> {
    // Function implementation here...
}
This means:

search_query: &'a str is a reference to a string slice with lifetime 'a.
You're returning a reference to a QueryOperations<'a>, meaning the QueryOperations struct and all its string references (&'a str inside queries) must live as long as 'a.
3. The Error: Temporary Values and Lifetimes:
The error you encountered comes from the fact that when you create new String values using .to_lowercase(), those are temporary values. If you try to store a reference to those Strings (&str), the references will point to values that will be dropped when the function ends, which causes lifetime issues.

For example, this:

rust
Copy code
.map(|query| query.to_lowercase().as_str())
is problematic because to_lowercase() returns a String, and as_str() just gives a reference to that temporary string. Once the loop iteration ends, the String is dropped, and your &str reference is invalid.

4. Solution: Owning the Strings
To avoid these lifetime issues, you need to own the Strings rather than referencing them. This is why I suggested changing the struct to:

rust
Copy code
pub struct QueryOperations {
    pub op: Ops,
    pub queries: Vec<String>,  // Now owns `String`s
}
By owning the strings, you're no longer dealing with references, and thus you don't need lifetime annotations. The function can then simply return a fully-owned QueryOperations struct.

Example of Working Code Without Lifetime Issues:
Struct Definition Without Lifetimes:
By owning the strings, you no longer need to worry about lifetimes:

rust
Copy code
pub struct QueryOperations {
    pub op: Ops,
    pub queries: Vec<String>,  // Owned Strings, no lifetimes needed
}
Function Returning an Owned Struct:
Since you're no longer returning references, you don't need to deal with lifetimes in the function signature:

rust
Copy code
pub fn parse_search_queries(search_query: &str) -> Result<QueryOperations, Error> {
    let mut queries: Vec<String> = Vec::new();  // Owned Strings

    if search_query.contains(AND_OPERATOR) {
        queries = search_query
            .split(AND_OPERATOR)
            .map(|query| query.to_lowercase())  // `to_lowercase()` returns a `String`
            .collect();
        Ok(QueryOperations { op: Ops::AND, queries })
    } else if search_query.contains(OR_OPERATOR) {
        queries = search_query
            .split(OR_OPERATOR)
            .map(|query| query.to_lowercase())
            .collect();
        Ok(QueryOperations { op: Ops::OR, queries })
    } else {
        queries.push(search_query.to_lowercase());
        Ok(QueryOperations { op: Ops::DEFAULT, queries })
    }
}
When to Use Lifetimes:
Use lifetimes when you're working with references (&'a str): If you're storing references to other values, you need to make sure those references live long enough, which is what lifetimes manage.
No lifetimes needed when using owned types (String): If you own the data (like using String instead of &str), then lifetimes are not necessary because the ownership model handles it.
By switching to Vec<String>, you're avoiding the complexity of dealing with references and lifetimes, since your QueryOperations struct now owns the data directly.
 */

#[warn(unused_variables,unused_parens,unused_assignments)]
pub fn parse_search_queries(search_query:& str) -> Result<QueryOperations,Error> {
    let mut queries:Vec<String> = Vec::new();
    if search_query.contains(AND_OPERATOR) {
        queries = search_query.split(AND_OPERATOR)
        .map(|query| query.to_lowercase())
            .collect();
        Ok(QueryOperations{op:Ops::AND,queries:queries})
    }
    else if search_query.contains(OR_OPERATOR) {
        queries = search_query.split(OR_OPERATOR)
        .map(|query| query.to_lowercase())
        .collect();
    Ok(QueryOperations{op:Ops::OR,queries:queries})
        
    }
    else {
        queries = Vec::new();
        queries.push(search_query.to_string());
        Ok(QueryOperations{op:Ops::DEFAULT,queries:queries})
    }
}