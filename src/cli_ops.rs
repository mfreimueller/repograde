use std::fmt::format;
use std::process::Command;

pub fn push_dir(path: &String) {
    let command_str = format!("pushd {}", path);
    exec_command(command_str);
}

pub fn pop_dir() {
    let command_str = String::from("popd");
    exec_command(command_str);
}

fn exec_command(command_str: String) -> bool {
    match Command::new("bash")
        .arg("-c")
        .arg(command_str)
        .spawn()
    {
        Ok(_child) => {
            true
        }
        Err(e) => {
            eprintln!("{}", e);
            false
        }
    }
}