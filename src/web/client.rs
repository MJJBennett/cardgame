use async_std::{
    io::copy,
    io::BufReader,
    io::BufRead,
    net::{TcpListener, TcpStream},
    prelude::*,
    task,
};
use structopt::StructOpt;

/* Web client functionality (an asynchronous library)
 *
 */

pub async fn async_tcp_test(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    println!("tcp_test: Connecting.");
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;
    println!("tcp_test: Writing message.");
    stream.write_all(b"Initial test message.").await?;
    stream.flush().await?;

    let mut s = String::new();
    println!("tcp_test: Receiving message.");
    match stream.read_to_string(&mut s).await {
        Err(e) => println!("Read error: {}", e),
        Ok(n) => println!("Read bytes: {}", n)
    };

    println!("Result message: '{}'", s);

    Ok(())
}

pub fn run_tcp_test(port: u16) -> bool {
    match task::block_on(async_tcp_test(port)) {
        Ok(_) => {
            println!("Successfully completed the TCP connection test.");
            true
        }
        Err(e) => {
            println!("Failure encountered during the TCP connection test: {}", e);
            false
        }
    }
}
