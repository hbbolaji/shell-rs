use std::{
    env,
    io::{Write, stdin, stdout},
    path::Path,
};

use is_executable::IsExecutable;

struct Executable {
    cmd: String,
    path: String,
}

fn main() {
    let shell_builtin = ["echo", "type", "exit"];

    loop {
        print!("$ ");
        stdout().flush().unwrap();

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();

        user_input = user_input.trim().to_string();
        let user_input_list = user_input.split(" ").collect::<Vec<&str>>();
        let command = user_input_list[0];
        let args = &user_input_list[1..];

        if command == "echo" {
            println!("{}", &user_input[5..].trim());
            continue;
        }

        if command == "type" {
            for arg in args {
                if shell_builtin.contains(arg) {
                    println!("{} is a shell builtin", arg);
                } else {
                    match find_executable(arg) {
                        Some(value) => println!("{}", value.path),
                        _ => println!("{} not found", arg),
                    };
                }
            }
            continue;
        }

        if command == "exit" {
            break;
        }

        let executable_cmd = find_executable(command);
        match executable_cmd {
            Some(value) => {
                let output = std::process::Command::new(value.cmd)
                    .args(args)
                    .status()
                    .unwrap();

                println!("{:?}", output.to_string());
            }
            None => println!("{}: command not found", command),
        }
    }
}

fn find_executable(executable: &str) -> Option<Executable> {
    let paths = env::var_os("PATH").unwrap();
    let path_list: Vec<&str> = paths.to_str().unwrap().split(":").collect();
    let path_dir: Vec<String> = path_list
        .iter()
        .map(|path| format!("{}/{}", path, executable))
        .filter(|item| Path::new(item).is_file() && Path::new(item).is_executable())
        .collect();

    if path_dir.is_empty() {
        return None;
    }

    Some(Executable {
        cmd: executable.into(),
        path: path_dir[0].to_string(),
    })
}
