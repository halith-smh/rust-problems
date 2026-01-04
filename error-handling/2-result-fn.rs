use std::fs;

fn main() {
    let file_content = read_file_result();
    match file_content {
        Ok(content) => {
            println!("File content: {}", content);
        },
        Err(err) => {
            println!("Error reading the file: {}", err.to_string());
        }
    }
}

fn read_file_result() -> Result<String, String>{
    let file_content = fs::read_to_string("example.txt");
    match file_content {
        Ok(content) => Ok(content),
        Err(err) => Err(err.to_string())
    }
}