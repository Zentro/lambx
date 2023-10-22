mod xeddsa;

use x25519_dalek::{PublicKey, StaticSecret};

use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::io;
use std::time::Duration;

fn main() -> Result<(), std::io::Error> {
    let server_address = "127.0.0.1:8888"; // Server address to bind to
    let client_address = "127.0.0.1:8889";

    let connect_address = "127.0.0.1:8887";

    thread::sleep(Duration::from_secs(2));

    // Server thread
    let server_thread = thread::spawn(move || {
        let server_socket = UdpSocket::bind(server_address).expect("Failed to bind server socket");

        println!("UDP server listening on {}", server_address);

        let mut buf = [0u8; 8192];

        loop {
            match server_socket.recv_from(&mut buf) {
                Ok((n, src)) => {
                    let received_data = String::from_utf8_lossy(&buf[..n]);
                    println!("Server: Received data from {}: {}", src, received_data);

                    // Send a "pong" response back to the client
                    let response_msg = "pong";
                    server_socket.send_to(response_msg.as_bytes(), src).ok();
                }
                Err(e) => {
                    eprintln!("Server: Error receiving data: {}", e);
                }
            }
        }
    });

    thread::sleep(Duration::from_secs(2));

    // Client thread
    let client_thread = thread::spawn(move || {
        let client_socket = UdpSocket::bind(client_address).expect("Failed to bind client socket");

        println!("UDP client sending to {}", connect_address);

        let mut message = String::new(); // Create a mutable string to store the user's message

        loop {
            // Read user input from the console
            io::stdin()
                .read_line(&mut message)
                .expect("Failed to read line");

            // Send the user's message to the server
            client_socket
                .send_to(message.as_bytes(), connect_address)
                .expect("Failed to send data");

            // Clear the message string for the next input
            message.clear();

            // Receive and display the response from the server
            let mut buf = [0u8; 8192];
            let (num_bytes, _src) = client_socket
                .recv_from(&mut buf)
                .expect("Failed to receive response");

            let response = String::from_utf8_lossy(&buf[..num_bytes]);
            println!("Client: Received response: {}", response);
        }

    });

    // Wait for both threads to finish
    server_thread.join().expect("Server thread panicked");
    client_thread.join().expect("Client thread panicked");

    Ok(())
}
