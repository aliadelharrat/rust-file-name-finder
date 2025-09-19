use std::{env, fs};

fn main() {
    if env::args().len() <= 2 {
        println!("Error: 2 arguments required — <file_path> <search_term>");
        std::process::exit(1);
    }

    let file_path = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();

    let file_contents = fs::read_to_string(&file_path).expect("Couldn't read file");

    for (line_number, line) in file_contents.lines().enumerate() {
        if line.trim().to_lowercase() == search_name.to_lowercase() {
            println!(
                "Found '{}' in '{}' at line {}",
                search_name,
                file_path,
                line_number + 1
            );
            return;
        }
    }

    println!("No match found");
    fs::write(&file_path, file_contents + search_name.as_str() + "\n").unwrap();
    println!("Added {search_name} to {file_path} successfully!");
}
