pub mod psswd_operations {
    use crate::input::read_input;
    use std::fs;
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
}
