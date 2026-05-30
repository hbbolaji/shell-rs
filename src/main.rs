use std::io::{ Write, stdin, stdout};

fn main() {

    let shell_builtin = ["echo", "type", "exit"];
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

        if command.starts_with("type") {
            let builtin = &command[5..].trim();

            if shell_builtin.contains(builtin) {
                println!("{} is a shell builtin", builtin);
            } else {
                println!("{} not found", builtin)
            }
            continue;
        }

        if command == "exit" {
            break;
        }

        let output = format!("{}: command not found", command);
        println!("{}", output)
    }
}
