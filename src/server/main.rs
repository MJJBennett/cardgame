use async_std::{
    io::copy,
    net::{TcpListener, TcpStream},
    prelude::*,
    task,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, help = "Supply the port number to host the server on.")]
    port: u16,
}

async fn async_main(port: u16) {
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)).await {
        Ok(l) => l,
        Err(e) => match port {
            // Okay, this formatting is... not very nice.
            x if x <= 1024 => panic!(format!(
                "{} | Note: This is most likely caused \
            by starting a web server on a port at or below 1024 without administrator \
            permissions.",
                e
            )),
            _ => panic!(format!("{}", e)),
        },
    };

    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        if let Ok(stream) = stream {
            let stream = stream;
            let (reader, writer) = &mut (&stream, &stream);
            if let Err(e) = copy(reader, writer).await {
                println!("Found a stream error! {}", e);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    println!("Starting CGServer.");
    task::block_on(async_main(args.port));

    Ok(())
}
