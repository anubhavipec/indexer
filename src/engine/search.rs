use std::collections::{HashMap, HashSet};


use super::model::{Ops, QueryOperations};
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

pub fn search<'a>(query_operations: &'a QueryOperations, index: &'a HashMap<String,HashSet<String>>) ->  HashSet<String> {

    let mut  result_hashsets= Vec::new();
    let mut ops_result_set= HashSet::new();
    if query_operations.op ==  Ops::DEFAULT && query_operations.queries.len() > 0 {
        /*
        we will just search the first word in our index and return, will implement full phrases search in future.
         */
            let query_string = query_operations.queries.get(0).unwrap();
            let result_set = index.get(query_string).unwrap();
            return result_set.clone();
    }
    
    else if query_operations.op == Ops::AND && query_operations.queries.len() > 0 {

    
        for token in query_operations.queries.clone() {

            let  result_set = index.get(token.as_str().trim()).cloned().unwrap_or_default();
            result_hashsets.push(result_set);     

        }
        if result_hashsets.len() == 1 {
            ops_result_set =  result_hashsets.get(0).cloned().unwrap();
            return ops_result_set;
        }

        ops_result_set = result_hashsets[0].clone();
        for set in result_hashsets.iter().skip(1){
            //Intersect all sets and return the result
            ops_result_set = ops_result_set
                            .intersection(set)
                            .cloned()
                            .collect(); 
        }
    }
    ops_result_set
}