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
        #[structopt(long = "server", short = "s", parse(from_os_str))]
        server: OsString,

        #[structopt(long = "username", short = "u", parse(from_os_str))]
        username: OsString,

        #[structopt(long = "password", short = "p", parse(from_os_str))]
        password: Option<OsString>,

        #[structopt(long = "private-key", short = "key", parse(from_os_str))]
        private_key_path: Option<OsString>,

        #[structopt(long = "dest", short = "d", parse(from_os_str))]
        destination: OsString,

        /// provide json output
        #[structopt(long = "json")]
        json: bool,
    },
}

fn main() {
    // Parse command-line arguments using StructOpt
    let args = Cli::from_args();

    match args {
        Cli::Copy {
            server,
            username,
            password,
            private_key_path,
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

            let server_str = server.to_str().ok_or("Failed to convert server OsString to str").unwrap();
            let server_with_port = ensure_port(server_str);

            let username_str = username.to_str().ok_or("Failed to convert username OsString to str").unwrap();

            let password_str = if let Some(p) = &password {
                p.to_str().map(|s| s.to_string()).ok_or("Failed to convert password OsString to str").unwrap()
            } else {
                String::from("")
            };

            println!("Server:   {}", &server_with_port);
            println!("Username: {}", username_str);
            println!("Password: {}", password_str);

            let mut session = session::session_with_password(
                username_str,
                &password_str,
                &server_with_port)
                .unwrap_or_else(|e| {
                    eprintln!("Failed to create session: {}", e);
                    std::process::exit(1);
                });

            println!("Connected");
            session.close();
        }
    }
}
