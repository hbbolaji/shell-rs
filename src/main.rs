use std::{
    env,
    io::{Write, stdin, stdout},
    path::Path,
};

use is_executable::IsExecutable;

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
                find_executable(&builtin);
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

fn find_executable(executable: &str) {
    let paths = env::var_os("PATH").unwrap();
    let path_list: Vec<&str> = paths.to_str().unwrap().split(":").collect();
    let path_dir: Vec<String> = path_list
        .iter()
        .map(|path| format!("{}/{}", path, executable))
        .filter(|item| Path::new(item).is_file() && Path::new(item).is_executable())
        .collect();

    if path_dir.is_empty() {
        println!("{} not found", executable);
        return;
    }

    println!("{}", &path_dir[0])
}
