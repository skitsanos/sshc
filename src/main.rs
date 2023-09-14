use std::ffi::OsString;

use structopt::StructOpt;

use crate::utils::ensure_port;

mod session;
mod utils;

#[derive(StructOpt, Debug)]
enum Cli {
    /// copy files to remote location
    #[structopt(help = "copy files to remote location")]
    Copy {
        #[structopt(long = "address", short = "a", parse(from_os_str))]
        address: OsString,

        #[structopt(long = "username", short = "u", parse(from_os_str))]
        username: OsString,

        #[structopt(long = "password", short = "p", parse(from_os_str))]
        password: Option<OsString>,

        #[structopt(long = "private-key", short = "key", parse(from_os_str))]
        private_key_path: Option<OsString>,

        #[structopt(long = "source", short = "s", parse(from_os_str))]
        source: OsString,

        #[structopt(long = "dest", short = "d", parse(from_os_str))]
        destination: OsString,

        /// provide json output
        #[structopt(long = "json")]
        json: bool,
    },

    Exec {
        #[structopt(long = "address", short = "a", parse(from_os_str))]
        address: OsString,

        #[structopt(long = "username", short = "u", parse(from_os_str))]
        username: OsString,

        #[structopt(long = "password", short = "p", parse(from_os_str))]
        password: Option<OsString>,

        #[structopt(long = "private-key", short = "key", parse(from_os_str))]
        private_key_path: Option<OsString>,

        #[structopt(long = "command", short = "c", parse(from_os_str))]
        command: OsString,
    },
}

fn main() {
    // Parse command-line arguments using StructOpt
    let args = Cli::from_args();

    match args {
        Cli::Copy {
            address,
            username,
            password,
            private_key_path,
            source,
            destination,
            json,
        } => {
            // Validation for either password or private_key_path
            if password.is_none() && private_key_path.is_none() {
                eprintln!("Error: You must provide either --password or --private-key");
                std::process::exit(1);
            }
            if password.is_some() && private_key_path.is_some() {
                eprintln!("Error: You can't provide both --password and --private-key. Choose one.");
                std::process::exit(1);
            }

            let address_str = address.to_str().ok_or("Failed to convert server OsString to str").unwrap();
            let address_with_port = ensure_port(address_str);

            let username_str = username.to_str().ok_or("Failed to convert username OsString to str").unwrap();

            let password_str = if let Some(p) = &password {
                p.to_str().map(|s| s.to_string()).ok_or("Failed to convert password OsString to str").unwrap()
            } else {
                String::from("")
            };

            let private_key_path_str = if let Some(p) = &private_key_path {
                p.to_str().map(|s| s.to_string()).ok_or("Failed to convert private key path OsString to str").unwrap()
            } else {
                String::from("")
            };

            let mut session = if !private_key_path_str.is_empty() {
                println!("connecting with {}", &private_key_path_str);
                session::session_with_key(username_str, &private_key_path_str, &address_with_port)
            } else if !password_str.is_empty() {
                println!("Connecting with username and password");
                session::session_with_password(username_str, &password_str, &address_with_port)
            } else {
                eprintln!("Error: Either a private key or a password must be provided.");
                std::process::exit(1);
            };

            if json {
                //println!("{}");
            } else {
                println!("Connected");
            }

            let source_str = source.to_str().ok_or("Failed to convert source OsString to str").unwrap();
            let destination_str = destination.to_str().ok_or("Failed to convert destination OsString to str").unwrap();

            println!("Copying {} to {} ...", source_str, destination_str);

            let scp = session.open_scp()
                .unwrap_or_else(|e| {
                    eprintln!("Failed to create SCP: {}", e);
                    std::process::exit(1);
                });

            scp.upload(source_str, destination_str)
                .unwrap_or_else(|e| {
                    eprintln!("Failed to copy: {}", e);
                    std::process::exit(1);
                });

            session.close();
        }

        Cli::Exec {
            address,
            username,
            password,
            private_key_path,
            command
        } => {
            let address_str = address.to_str().ok_or("Failed to convert server OsString to str").unwrap();
            let address_with_port = ensure_port(address_str);

            let username_str = username.to_str().ok_or("Failed to convert username OsString to str").unwrap();

            let password_str = if let Some(p) = &password {
                p.to_str().map(|s| s.to_string()).ok_or("Failed to convert password OsString to str").unwrap()
            } else {
                String::from("")
            };

            let command_str = command.to_str().ok_or("Failed to convert command OsString to str").unwrap();

            let private_key_path_str = if let Some(p) = &private_key_path {
                p.to_str().map(|s| s.to_string()).ok_or("Failed to convert private key path OsString to str").unwrap()
            } else {
                String::from("")
            };

            let mut session = if !private_key_path_str.is_empty() {
                println!("connecting with {}", &private_key_path_str);
                session::session_with_key(username_str, &private_key_path_str, &address_with_port)
            } else if !password_str.is_empty() {
                println!("Connecting with username and password");
                session::session_with_password(username_str, &password_str, &address_with_port)
            } else {
                eprintln!("Error: Either a private key or a password must be provided.");
                std::process::exit(1);
            };

            let exec = session.open_exec()
                .unwrap_or_else(|e| {
                    eprintln!("Failed to create SSH_EXEC: {}", e);
                    std::process::exit(1);
                });

            let vec: Vec<u8> = exec.send_command(&command_str).unwrap();
            println!("{}", String::from_utf8(vec).unwrap());

            session.close()
        }
    }
}
