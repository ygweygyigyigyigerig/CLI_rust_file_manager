mod input;
mod password_operations;

use crate::input::read_input;
use crate::password_operations::psswd_operations;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

fn load_saved_dir_path() -> io::Result<PathBuf> {
    fs::read_to_string("saved_dir").map(PathBuf::from)
}

fn check_for_dir_save(dir: &mut PathBuf, is_dir_set_true: &mut bool) {
    match fs::exists("saved_dir") {
        Ok(true) => {
            println!("Saved directory dected, would you like to load it?");
            let user_choice = read_input::read_str().to_lowercase();

            if user_choice == "yes" || user_choice == "y" {
                *dir = load_saved_dir_path().expect("e");
                *is_dir_set_true = true;
            }
        }
        Ok(false) => println!("\n"),
        _ => println!("Error"),
    }
}

fn set_dir() -> PathBuf {
    println!("Chose directory, where passwords will be saved ");

    loop {
        let mut dir: PathBuf = read_input::read_str().into();
        if Path::new(&dir).exists() {
            println!("Path '{}' exists! Path updated sucesfully.", dir.display());
            println!(
                "Would you like to save directory, so it will be loaded automatically next time? (Yes/No)"
            );
            let user_choice = read_input::read_str().to_lowercase();

            if user_choice == "yes" || user_choice == "y" {
                save_dir(&mut dir);
            }
            return dir;
        } else {
            println!("Path '{}' does not exist! Retry\n", dir.display());
            continue;
        }
    }
}

fn save_dir(dir: &mut PathBuf) {
    fs::write("saved_dir", dir.as_os_str().as_encoded_bytes()).expect("crash");
}

fn main() -> std::io::Result<()> {
    let mut is_dir_set = false;
    let mut dir = PathBuf::new();
    check_for_dir_save(&mut dir, &mut is_dir_set);

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
        let action: u32 = read_input::read_int()?;

        match action {
            1 => {
                if is_dir_set {
                    psswd_operations::create_passwd(&dir)?
                } else {
                    println!("Set directory where passwords will be saved (Option 4)");
                }
            }

            2 => {
                if is_dir_set {
                    psswd_operations::remove_psswd(&dir)?
                } else {
                    println!("Set directory in which passwords will be removed (Option 4");
                }
            }

            3 => {
                if is_dir_set {
                    psswd_operations::list_passwds(&dir)?
                } else {
                    println!("Set directory");
                }
            }

            4 => {
                dir = set_dir();
                is_dir_set = true;
            }

            5 => break,
            _ => println!("Chose from 1-5"),
        }
    }
    Ok(())
}
