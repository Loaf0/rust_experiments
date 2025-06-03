#![allow(unused)]

use std::{fs::{self, File}, io::{Read, Write}, path};
use std::str::FromStr;

fn file_editor() {
    // let mut file: File = fs::File::create("save_state.txt").unwrap();
    // file.write_all("Hello World!".as_bytes()).unwrap();
    let file_path: String = "save_state.txt".to_string();
    let mut contents: String = read_file(file_path.to_string());

    
    println!("The file contains :\n{}\nEdit file? (Y/N)", contents);

    let mut user_input: String = String::new();
    let mut looping:bool = true;
    
    while looping {
        user_input = input::<String>().unwrap().to_uppercase();

        match user_input.as_str().trim() {
            "N" => {
                looping = false;
            }
            "Y" => {
                looping = false;
                println!("Enter new file contents below\n");
                contents = input::<String>().unwrap();
                println!("Saving Contents to {} \n{}", file_path, contents);
                fs::write(file_path.to_string(), contents.as_bytes()).unwrap(); //alternative faster to code
            }
            _ => {
                println!("You entered: {}", user_input);
            }
        };
    }
    println!("Goodbye!");

}

fn read_file(file_path: String) -> String {
    let mut file: File = fs::File::open(file_path.to_string()).unwrap();
    let mut file_contents: String = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    file_contents
}

fn input<T: FromStr>() -> Result<T, <T as FromStr>::Err> {
    let mut input: String = String::with_capacity(64); 
    
    std::io::stdin()
    .read_line(&mut input)
    .expect("Input could not be read");
    
    input.parse()
}