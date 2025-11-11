use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

fn set_dir() -> std::io::Result<()> {
    println!("Chose directory, where passwords will be saved ");

    loop {
        let input = read_input_str();

        let path = &input;

        if Path::new(&path).exists() {
            println!("Path '{}' exists! Path updated sucesfully.", path);
            break;
        } else {
            println!("Path '{}' does not exist! Retry", path);
            continue;
        }
    }
    Ok(())
}

fn set_path_and_check_if_exists() -> io::Result<PathBuf> {
    let dir = PathBuf::from("/home/simon/secret-passwords");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn read_input_int() -> io::Result<u32> {
    let mut input = String::new();
    let stdin = io::stdin();

    loop {
        input.clear();
        stdin.read_line(&mut input)?;

        if let Ok(num) = input.trim().parse::<u32>() {
            break Ok(num);
        }

        println!("Invalid input! Try again.");
    }
}

fn read_input_str() -> String {
    let mut input = String::new();
    let stdin = io::stdin();

    loop {
        match stdin.read_line(&mut input) {
            Ok(_) => return input.trim().to_string(),
            Err(e) => {
                println!("Error reading input: {}. Try again.", e);
                continue;
            }
        }
    }
}

fn create_passwd() -> std::io::Result<()> {
    println!("Chose name for your password: ");
    let name_of_passwd: String = read_input_str();

    let dir = set_path_and_check_if_exists()?;
    let path = dir.join(&name_of_passwd);

    println!("Your password: ");
    let passwd = read_input_str();
    fs::write(&path, &passwd)?;
    Ok(())
}

fn remove_psswd() -> std::io::Result<()> {
    // Intialize dir, to later join name_of_passwd_to_del later on.
    let dir = set_path_and_check_if_exists()?;

    println!("Enter name of password you'd like to delete: ");
    let name_of_passwd_to_del: String = read_input_str();
    let passwd = dir.join(&name_of_passwd_to_del);
    fs::remove_file(&passwd)?;
    Ok(())
}

fn list_passwds() -> std::io::Result<()> {
    let path = set_path_and_check_if_exists()?;
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }
    Ok(())
}
fn main() -> std::io::Result<()> {
    println!("***************************");
    println!("Welcome in password manager");
    println!("***************************");

    println!("1. Create password");
    println!("2. Remove password");
    println!("3. List all passwords");
    println!("4. Set directory");
    println!("5. Exit\n");
    loop {
        println!("Chose your action: ");
        let action: u32 = read_input_int()?;

        match action {
            1 => create_passwd()?,
            2 => remove_psswd()?,
            3 => list_passwds()?,
            4 => set_dir()?,
            5 => break,
            _ => println!("Chose from 1-4"),
        }
    }
    Ok(())
}
