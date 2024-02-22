use std::net::TcpStream;
use ssh::LocalSession;

pub fn session_with_key(username: &str, private_key_path: &str, server: &str) -> LocalSession<TcpStream> {
    ssh::create_session()
        .username(username)
        .private_key_path(private_key_path)
        .connect(server)
        .unwrap_or_else(|e| {
            println!("{}", e.to_string());
            eprintln!("Failed to create session: {}", e);
            std::process::exit(1);
        })
        .run_local()
}

pub fn session_with_password(username: &str, password: &str, server: &str) -> LocalSession<TcpStream> {
    ssh::create_session()
        .username(username)
        .password(password)
        .connect(server)
        .unwrap_or_else(|e| {
            println!("{}", e.to_string());
            eprintln!("Failed to create session: {}", e);
            std::process::exit(1);
        })
        .run_local()
}