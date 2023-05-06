use std::io;
use std::io::Write;
use std::fs;
use std::path::{Path, PathBuf};
use sha256::digest;

//.pse is the file extension for passala
fn adder(password:&str, filename:&str, website:&str, username:&str) -> std::io::Result<()> {
    let dir_path = PathBuf::from("C:/Users/user/Passala");
    let file_path = dir_path.join(filename.to_owned() + ".pse");
    if !dir_path.exists() {
        fs::create_dir(&dir_path)?;
    }
    let password_hashed = digest(password);
    let website_hashed= digest(website);
    let username_hashed:String = digest(username);
    let mut file = fs::OpenOptions::new().append(true).create(true).open(&file_path)?;
    writeln!(file, "{}", password_hashed)?;
    writeln!(file, "{}", website_hashed)?;
    writeln!(file, "{}", username_hashed)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Order to add a password please write: ADD. Order to Dump the hashed passwords write: DMP: ");
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).expect("Err");
    let mode = mode.trim();

    let mut filename:String = String::new();

    if mode == "DMP" {
        println!("Enter Filename: ");
        io::stdin().read_line(&mut filename).expect("Err");
        let filename:&str = filename.trim();

        let readble = "C:/Users/user/Passala/".to_owned() + &filename + ".pse";
        let contents = fs::read_to_string(readble)?;
        println!("{}", contents);
    } else if mode == "ADD" {
        println!("Enter Filename: ");
        io::stdin().read_line(&mut filename).expect("Err");
        let filename:&str = filename.trim();

        println!("Enter website name: ");
        let mut website:String = String::new();
        io::stdin().read_line(&mut website).expect("Err");
        let website:&str = website.trim();

        println!("Enter Username: ");
        let mut username:String = String::new();
        io::stdin().read_line(&mut username).expect("Err");
        let username:&str = username.trim();

        println!("Enter a password: ");
        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Err");
        let password:&str = password.trim();
        adder(password, filename, website, username)?;
    }
    Ok(())
}
