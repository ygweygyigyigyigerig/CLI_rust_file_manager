pub mod psswd_and_dirs {
    use crate::input::read_input;
    use std::fs;
    use std::io;
    use std::path::Path;
    use std::path::PathBuf;

    pub fn create_passwd(dir: &PathBuf) -> std::io::Result<()> {
        println!("Chose name for your password: ");
        let name_of_passwd: String = read_input::read_str();

        let path = dir.join(&name_of_passwd);

        println!("Your password: ");
        let passwd = read_input::read_str();
        fs::write(&path, &passwd)?;
        println!("Sucesfully saved password at {}", path.display());
        Ok(())
    }

    pub fn remove_psswd(dir: &PathBuf) -> std::io::Result<()> {
        // Intialize dir, to later join name_of_passwd_to_del later on.
        println!("Enter name of password you'd like to delete: ");
        // get input
        let name_of_passwd_to_del: String = read_input::read_str();
        // get whole password path
        let passwd_path = dir.join(&name_of_passwd_to_del);
        fs::remove_file(&passwd_path)?;
        println!("Sucesfully deleted password, {}", passwd_path.display());
        Ok(())
    }

    pub fn list_passwds(dir: &PathBuf) -> std::io::Result<()> {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            println!("{}", path.display());
        }
        Ok(())
    }

    pub fn load_saved_dir_path() -> io::Result<PathBuf> {
        fs::read_to_string("saved_dir").map(PathBuf::from)
    }

    pub fn check_for_dir_save(dir: &mut PathBuf, is_dir_set_true: &mut bool) {
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

    pub fn set_dir() -> PathBuf {
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

    pub fn save_dir(dir: &mut PathBuf) {
        fs::write("saved_dir", dir.as_os_str().as_encoded_bytes()).expect("crash");
    }
}
