use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::io::{self, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::thread;

// This function generates a short code using alphanumeric characters
fn generate_short_code(length: usize) -> String {
    let mut rng = thread_rng();
    let code: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(length)
        .collect();
    code
}

fn main() -> io::Result<()> {
    // Create a HashMap to store the short codes and their corresponding URLs
    let mut urls: HashMap<String, String> = HashMap::new();

    // Set up the server to listen on port 8080
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080...");

    // Handle incoming connections
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0; 1024];

        // Read the request from the client
        stream.read(&mut buffer)?;

        // Parse the request
        let request = String::from_utf8_lossy(&buffer[..]);
        let parts: Vec<&str> = request.split(" ").collect();

        if parts.len() > 1 && parts[0] == "GET" {
            // Get the short code from the request
            let path = parts[1];
            let path_parts: Vec<&str> = path.split("/").collect();
            let short_code = path_parts[path_parts.len() - 1];

            // Look up the URL corresponding to the short code
            if let Some(url) = urls.get(short_code) {
                // Redirect the client to the URL
                let response = format!("HTTP/1.1 301 Moved Permanently\r\nLocation: {}\r\n\r\n", url);
                stream.write(response.as_bytes())?;
            } else {
                // If the short code is not found, return a 404 error
                let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                stream.write(response.as_bytes())?;
            }
        } else if parts.len() > 1 && parts[0] == "POST" {
            // Get the URL from the request body
            let url = parts[parts.len() - 1].to_string();

            // Generate a short code for the URL
            let short_code = generate_short_code(6);

            // Store the URL and its corresponding short code in the HashMap
            urls.insert(short_code.clone(), url.clone());

            // Send the short code back to the client
            let response = format!("HTTP/1.1 200 OK\r\n\r\nhttp://127.0.0.1:8080/{}", short_code);
            stream.write(response.as_bytes())?;
        } else {
            // If the request is not recognized, return a 400 error
            let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
            stream.write(response.as_bytes())?;
        }

        stream.flush()?;
    }
