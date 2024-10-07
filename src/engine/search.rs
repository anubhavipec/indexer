use std::collections::HashMap;

use super::tokenizer::tokenize;

/*
Why lifetime is  needed in this case:

The function could return references to String elements stored inside the index. 
If the lifetime wasn't specified, Rust wouldn’t know how long those references should live, 
potentially allowing you to return a reference to data that might not exist after the function scope ends.
In this case, Vec<&'a String> means that the function is promising that the references it returns will live at least as long as the index and query,
 hence the need for the lifetime annotation.
Why Vec<usize> wouldn’t need lifetimes:
If the index is of type HashMap<String, Vec<usize>> and the return type is Vec<usize>, no lifetime annotations will be required. Here’s why:

Key Difference: Owned Data (Vec<usize>) vs. Borrowed Data (&String)
Owned Data (Vec<usize>):

In this case, usize is a simple, copyable type (like integers). When you retrieve usize values from the index, you're copying the values and not borrowing them.
Since usize is being moved or copied into the return Vec<usize>, there are no references to manage, and therefore no need for lifetimes.
Borrowed Data (Vec<&String>):

In the previous example, when returning a Vec<&'a String>, you're borrowing the String references from the index, meaning the function needs to ensure that the borrowed references remain valid for as long as they're being used. Hence, lifetimes are necessary.
If the return type were Vec<usize>, it would be a collection of values, not references. Values don't have lifetimes attached to them in the same way as references because they are copied and not borrowed from the original data structure.
 */

pub fn search<'a>(query: &'a str, index: &'a HashMap<String,Vec<String>>) -> Vec<&'a String> {

    let mut search_result = Vec::new();

    let query_tokens = tokenize(query," "); 

    for token in query_tokens{

        if let Some(doc) = index.get(&token) {
            search_result.extend(doc);
        }
    }

    search_result
}