mod cli;
mod crypto;

use cli::cli;
use crypto::encrypt::{decrypt_file, derive_key, encrypt_file};
use std::env;

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("encrypt", sub_matches)) => {
            let input = sub_matches
                .get_one::<String>("input")
                .expect("Input file is required");
            let password = sub_matches
                .get_one::<String>("password")
                .expect("Password is required");
            let salt =
                env::var("LOCKUR_SALT").unwrap_or_else(|_| String::from("your_unique_salt_here"));
            let key = derive_key(password, salt.as_bytes());

            if let Err(e) = encrypt_file(input, &key) {
                eprintln!("Encryption failed: {}", e);
                std::process::exit(1);
            }
            println!("File encrypted successfully.");
        }
        Some(("decrypt", sub_matches)) => {
            let input = sub_matches
                .get_one::<String>("input")
                .expect("Input file is required");
            let password = sub_matches
                .get_one::<String>("password")
                .expect("Password is required");
            let salt =
                env::var("LOCKUR_SALT").unwrap_or_else(|_| String::from("your_unique_salt_here"));
            let key = derive_key(password, salt.as_bytes());

            if let Err(e) = decrypt_file(input, &key) {
                eprintln!("Decryption failed: {}", e);
                std::process::exit(1);
            }
            println!("File decrypted successfully.");
        }
        Some(("set", sub_matches)) => {
            let salt = sub_matches
                .get_one::<String>("salt")
                .expect("Salt value is required");
            env::set_var("LOCKUR_SALT", salt);
            println!("Salt value set successfully.");
        }
        _ => {
            eprintln!("Invalid command");
            std::process::exit(1);
        }
    }
}
