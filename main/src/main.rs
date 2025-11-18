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
            let user_choice = read_input_str().to_lowercase();

            if user_choice == "yes" || user_choice == "y" {
                *dir = load_saved_dir_path().expect("e");
                *is_dir_set_true = true;
            }
        }
        Ok(false) => println!("\n"),
        _ => println!("Error"),
    }
}

fn save_dir(dir: &mut PathBuf) {
    fs::write("saved_dir", dir.as_os_str().as_encoded_bytes());
}
fn set_dir() -> PathBuf {
    println!("Chose directory, where passwords will be saved ");

    loop {
        let mut dir: PathBuf = read_input_str().into();
        if Path::new(&dir).exists() {
            println!("Path '{}' exists! Path updated sucesfully.", dir.display());
            println!(
                "Would you like to save directory, so it will be loaded automatically next time? (Yes/No)"
            );
            let user_choice = read_input_str().to_lowercase();

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

fn create_passwd(dir: &PathBuf) -> std::io::Result<()> {
    println!("Chose name for your password: ");
    let name_of_passwd: String = read_input_str();

    let path = PathBuf::from(dir).join(&name_of_passwd);

    println!("Your password: ");
    let passwd = read_input_str();
    fs::write(&path, &passwd)?;
    println!("Sucesfully saved password at {}", path.display());
    Ok(())
}

fn remove_psswd(dir: &PathBuf) -> std::io::Result<()> {
    // Intialize dir, to later join name_of_passwd_to_del later on.
    println!("Enter name of password you'd like to delete: ");
    // get input
    let name_of_passwd_to_del: String = read_input_str();
    // get whole password path
    let passwd_path: PathBuf = PathBuf::from(dir).join(&name_of_passwd_to_del);
    fs::remove_file(&passwd_path)?;
    println!("Sucesfully deleted password, {}", passwd_path.display());
    Ok(())
}

fn list_passwds(dir: &PathBuf) -> std::io::Result<()> {
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }
    Ok(())
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
        let action: u32 = read_input_int()?;

        match action {
            1 => {
                if is_dir_set {
                    create_passwd(&dir)?
                } else {
                    println!("Set directory where passwords will be saved (Option 4)");
                }
            }

            2 => {
                if is_dir_set {
                    remove_psswd(&dir)?
                } else {
                    println!("Set directory in which passwords will be removed (Option 4");
                }
            }

            3 => {
                if is_dir_set {
                    list_passwds(&dir)?
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
