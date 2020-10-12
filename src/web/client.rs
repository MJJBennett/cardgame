use async_std::{
    io::copy,
    io::BufRead,
    io::BufReader,
    net::{TcpListener, TcpStream},
    prelude::*,
    task,
};
use structopt::StructOpt;

use super::web_strings::*;

/* Web client functionality (an asynchronous library)
 *
 */

pub async fn async_tcp_test(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    println!("tcp_test: Connecting.");
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;
    println!("tcp_test: Writing message.");
    write_string(&mut stream, String::from("Initial test message.")).await?;

    println!("tcp_test: Receiving message.");
    let ret = match read_string(&mut stream).await {
        Err(e) => {
            println!("Read error: {}", e);
            return Err(e);
        }
        Ok((n, s)) => {
            println!("Read bytes: {}", n);
            s
        }
    };

    println!("Result message: '{}'", ret);

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
