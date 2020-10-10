use std::env;
use std::io;
use std::path::PathBuf;

pub fn get_cwd() -> io::Result<PathBuf> {
    let mut pwd = env::current_exe()?;
    pwd.pop();
    Ok(pwd)
}

pub fn get_main() -> io::Result<PathBuf> {
    let exe = env::current_exe()?;
    Ok(exe)
}
pub fn get_system(){
    let mut os = 0;
    let mut version = 0;
    match env::consts::OS {
    "linux" => os = 1,
    "windows" => os = 2,
    "macos" => os = 3,
    _ => os = 4
    };

    println!("Operating System Code: {}", os);
}
