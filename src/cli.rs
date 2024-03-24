use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("lockur")
        .about("A simple cli tool to make file encryption a breeze.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("encrypt")
                .about("Encrypt file")
                .arg(Arg::new("input")
                    .required(true))
                .arg(Arg::new("password")
                    .long("password")
                    .short('p')
                    .help("Password for encryption")
                    .required(true)),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypt file")
                .arg(Arg::new("input")
                    .required(true))
                .arg(Arg::new("password")
                    .long("password")
                    .short('p')
                    .help("Password for decryption")
                    .required(true)),
        )
        .subcommand(
            Command::new("set")
                .about("Set configuration options")
                .arg(Arg::new("salt")
                    .long("salt")
                    .value_name("SALT")
                    .help("Set the salt value for key derivation")
                    .required(true)),
        )
}