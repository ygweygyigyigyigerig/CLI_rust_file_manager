mod input;
mod passwords_and_dirs_operations;
use crate::input::read_input;
use crate::passwords_and_dirs_operations::psswd_and_dirs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let mut is_dir_set = false;
    let mut dir = PathBuf::new();
    psswd_and_dirs::check_for_dir_save(&mut dir, &mut is_dir_set);

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
                    psswd_and_dirs::create_passwd(&dir)?
                } else {
                    println!("Set directory where passwords will be saved (Option 4)");
                }
            }

            2 => {
                if is_dir_set {
                    psswd_and_dirs::remove_psswd(&dir)?
                } else {
                    println!("Set directory in which passwords will be removed (Option 4");
                }
            }

            3 => {
                if is_dir_set {
                    psswd_and_dirs::list_passwds(&dir)?
                } else {
                    println!("Set directory");
                }
            }

            4 => {
                dir = psswd_and_dirs::set_dir();
                is_dir_set = true;
            }

            5 => break,
            _ => println!("Chose from 1-5"),
        }
    }
    Ok(())
}
