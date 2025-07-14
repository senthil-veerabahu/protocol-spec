use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpSocket};

use std::io::{self, Cursor};

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080".parse().unwrap();

    let socket = TcpSocket::new_v4()?;
    let mut stream = socket.connect(addr).await?;
    let mut buffer = Cursor::new(b"GET /product/1 HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/8.7.1\r\nAccept: */*\r\n\r\n");
    stream.write_buf(&mut buffer).await?;
    stream.flush().await?;
    println!("Message sent to server");
    // Here you would typically read a response from the server
    // For example:
    let mut full_buffer = vec![0; 0];
    loop{
        let mut buffer = vec![0; 0];
        let n = stream.read_buf(&mut buffer).await?;
        if n == 0 {
            break; // No more data to read
        }else{
            full_buffer.extend_from_slice(&buffer[..n]);
        }
    }
    println!("Received response: {}", String::from_utf8_lossy(&full_buffer[..full_buffer.len() ])); // Exclude the last byte which is usually a null terminator
    // Close the stream
    stream.shutdown().await?;
    println!("Connection closed");
    // Keep the main function alive to allow for async operations
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("Client finished execution");
    // Return Ok to indicate successful execution

    Ok(())
}