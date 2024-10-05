use engine::tokenizer::tokenize;

mod engine;

fn main() {
    println!("Hello, world!");

    match tokenize("/Users/anubhav/Documents/rust-test/") {
        Ok(tokens) => {
            println!("{}",tokens.len());
            // for token in tokens{
            //     println!("{:?}",token);
            // }
        }
        Err(err) => {
            eprintln!("Error Occurred {}",err);
        }
        
    }
}
