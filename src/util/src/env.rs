use std::env;
use std::io;
use std::path::PathBuf;
use std::process::Command;


pub fn get_cwd() -> io::Result<PathBuf> {
    let mut pwd = env::current_exe()?;
    pwd.pop();
    Ok(pwd)
}

pub fn get_main() -> io::Result<PathBuf> {
    let exe = env::current_exe()?;
    Ok(exe)
}

pub fn get_system() {
    let mut os = 0;
    let mut version = None;
    let nomatch = &[0];
    match env::consts::OS {
        "linux" => {
            os = 1;
            version = Some(Command::new("uname")
                .arg("-r")
                .output()
                .expect("Failed to find Linux kernel version"));
        }
        "windows" => {
            os = 1;
            version = Some(Command::new("systeminfo")
                .arg("| findstr /B /C:\"OS Name\" /C:\"OS Version\"")
                .output()
                .expect("Failed to find Windows version"));

        }
        "macos" => {
            os = 3;
            version = Some(Command::new("uname")
                .arg("-r")
                .output()
                .expect("Failed to find MacOS version"));
            
        }
        _ => os = 4,
    };
    let _version = 
    match &version {
        Some(p) => String::from_utf8_lossy(&p.stdout),
        None => String::from_utf8_lossy(nomatch)
    };
    println!("OS = {} Version = {}", os, _version);
}
