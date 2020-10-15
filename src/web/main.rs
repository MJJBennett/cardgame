use async_std::{
    io::copy,
    net::{TcpListener, TcpStream},
    prelude::*,
    task,
};
use structopt::StructOpt;
use cardgame::web::web_strings::*;
use cardgame::web::proto::*;

/* Web server functionality
 *
 */

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, help = "Supply the port number to host the server on.", default_value = "3124")]
    port: u16,
    #[structopt(long, help = "Run a test server.")]
    test: bool, 
    #[structopt(short, long, help = "Supply the IP address to host the server on.", default_value = "127.0.0.1")]
    ip: String,
}

async fn test_main(ip: String, port: u16) {
    let listener = match TcpListener::bind(format!("{}:{}", ip, port)).await {
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
            println!("test_main: Copying message over.");
            if let Err(e) = copy(reader, writer).await {
                println!("Found a stream error! {}", e);
            }
            println!("test_main: Finished copying message(s).");
        }
    }
}

async fn accept_handler(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, writer) = &mut (&stream, &stream);
    /*if let Err(e) = copy(reader, writer).await {
        println!("Found a stream error! {}", e);
    }*/
    let res = read_string(reader).await?.1;
    println!("Server got info: {}", res);
    if let Some(s) = res.split(';').next() {
        if s == TcpStream::version_string() {
            println!("Successful connection!");
            write_string(writer, String::from("You are connected!")).await?;
        }
        else {
            println!("Wrong version!");
            write_string(writer, String::from("Your version is incorrect!")).await?;
        }
    }
    else {
        println!("Unsuccessful connection!");
        write_string(writer, String::from("You are not connected!")).await?;
    }

    write_string(writer, String::from("Goodbye.")).await
}

async fn async_main(ip: String, port: u16) {
    let listener = match TcpListener::bind(format!("{}:{}", ip, port)).await {
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
            task::spawn(async move {
                if let Err(e) = accept_handler(stream).await {
                    println!("Connection encoutered error: {}", e);
                }
            });
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    println!("Running on {}:{}.", args.ip, args.port);

    if args.test {
        println!("Running test server!");
        task::block_on(test_main(args.ip, args.port));
        return Ok(());
    }

    println!("Starting CGServer.");
    task::block_on(async_main(args.ip, args.port));

    Ok(())
}
