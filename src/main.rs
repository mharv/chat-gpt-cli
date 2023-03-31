use clap::Parser;
use std::{fs::File, io::Read};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// chat gpt prompt
    prompt: Vec<String>,
}

fn main() {
    // check if api key exists
    let mut api_key_file = File::open("./.env").unwrap();
    let mut api_key = String::new();

    api_key_file.read_to_string(&mut api_key).unwrap();

    println!("{}", api_key);


    let args = Args::parse();
    let prompt = args.prompt.join(" ");
    println!("{}", prompt);


    // do a chatGPT api call
}
