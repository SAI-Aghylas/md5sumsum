extern crate walkdir;
use std::{env, process::Command};

fn main() {
    let mut hashcat = String::from("");
    for arg in env::args().skip(1) {
        for entry in walkdir::WalkDir::new(arg)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            let mut p =entry.path().to_string_lossy();
            let mut cmd = Command::new("sh");
            cmd.arg("-c").arg(format!("md5sum \"{}\"", p));
            let mut output = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();
            let mut vec: Vec<&str> = output.split(" ").collect();
            hashcat += vec[0];
        }
    }
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(format!("echo \"{}\" | md5sum", hashcat));
    let output = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();
    print!("{}", output);
}
