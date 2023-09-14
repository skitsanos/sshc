pub fn ensure_port(server: &str) -> String {
    let parts: Vec<&str> = server.split(':').collect();

    match parts.len() {
        1 => format!("{}:22", parts[0]), // If there's only a host, append the default port
        2 => server.to_string(),         // If there's a host and a port, return as is
        _ => {
            eprintln!("Invalid server format. Please use 'host:port' format.");
            std::process::exit(1);
        }
    }
}
