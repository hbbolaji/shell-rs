use std::io::{ Write, stdin, stdout};

fn main() {
    loop {
        print!("$ ");
        stdout().flush().unwrap();
        
        let mut command = String::new();
        stdin().read_line(&mut command).unwrap();

        command = command.trim().to_string();

        if command.starts_with("echo") {
            println!("{}", &command[5..].trim());
            continue;
        }

        if command == "exit" {
            break;
        }

        let output = format!("{}: command not found", command);
        println!("{}", output)
    }
}
