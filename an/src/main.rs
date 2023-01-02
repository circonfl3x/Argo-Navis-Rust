use std::fs::File;
use std::io::*;
use std::process::exit;
mod file_management;
mod memory_management;
mod parser;
mod tokenizer;
mod base_lib {
    pub mod stdout;
}
mod useful_stuff;
fn main() {
    let filepath: String = String::from("C:/Users/Administrator/Desktop/Projets/an/src/test");
    match File::open(&filepath) {
        Ok(_) => (),
        Err(_) => panic!("file doesn't exist"),
    };

    if File::metadata(&File::open(&filepath).unwrap())
        .unwrap()
        .is_dir()
    {
        panic!("Filepath is a directory, not a file");
    }

    match File::create(".argo_cache") {
        Ok(t) => t,
        Err(_) => panic!("Couldn't create cache file!"),
    };
    let filemanagement = file_management::Fs::new(&filepath);

    let lines: Vec<String> = file_management::Fs::open_file(&filemanagement);
    let mut line_number = 1;
    for l in lines {
        if l.is_empty() {
            line_number += 1;
        } else {
            let token = tokenizer::Tokenizer::new(&line_number, &l);
            let operator: String;
            let var: String;
            let value: String;
            (operator, var, value) = token.experimental_seperate();

            let parse = parser::Parser::new(&line_number, &operator, &value, &var, &filepath);
            parse.function();
        }
    }

    //File::create(".argo_cache").expect("Failed to clear cache");
}
