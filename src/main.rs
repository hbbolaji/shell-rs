use std::io::{ Write, stdin, stdout};

fn main() {
    print!("$ ");
    stdout().flush().unwrap();
    
    let mut command = String::new();
    stdin().read_line(&mut command).unwrap();
    let output = format!("{}: command not found", command.trim_end());
    println!("{}", output)
}
