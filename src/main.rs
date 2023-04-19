use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use sha256::digest;

fn adder(password:&str) -> std::io::Result<()> {
    let dir_path = PathBuf::from("C:/Users/user/Passala");
    let file_path = dir_path.join("hash.pse");
    if !dir_path.exists() && !file_path.exists() {
        fs::create_dir(&dir_path)?;
        fs::File::create(&file_path)?;
        let password_hashed = digest(password);
        fs::write(file_path, password_hashed)?;
    } else {
        let password_hashed = digest(password);
        fs::write(file_path, password_hashed)?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Order to add a password please write: ADD. Order to Dump the hashed passwords write: DMP");
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).expect("Err");
    let mode = mode.trim();

    if mode == "DMP" {
        let readble = "C:/Users/user/Passala/hash.pse";
        let contents = fs::read_to_string(readble)?;
        println!("{}", contents);
    } else if mode == "ADD" {
        println!("Enter a password: ");
        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Err");
        let password:&str = password.trim();
        adder(password)?;
    }
    Ok(())
}