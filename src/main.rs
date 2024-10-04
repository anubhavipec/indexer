use engine::tokenizer::tokenize;

mod engine;

fn main() {
    println!("Hello, world!");

    match tokenize("/Users/anubhav/Documents/token-test/") {
        Ok(tokens) => {
            for token in tokens{
                println!("{:?}",token);
            }
        }
        Err(err) => {
            eprintln!("Error Occurred {}",err);
        }
        
    }
}
