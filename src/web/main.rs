use async_std::{
    io::copy,
    net::{TcpListener, TcpStream},
    prelude::*,
    task,
};
use cardgame::web::proto::*;
use cardgame::web::web_strings::*;
use structopt::StructOpt;
use cardgame::game;

/* Web server functionality
 *
 */

/* Cli
 *
 * This data structure represents (and handles) command line
 * arguments provided to the server.
 */
#[derive(StructOpt)]
struct Cli {
    #[structopt(
        short,
        long,
        help = "Supply the port number to host the server on.",
        default_value = "3124"
    )]
    port: u16,
    #[structopt(long, help = "Run a test server.")]
    test: bool,
    #[structopt(
        short,
        long,
        help = "Supply the IP address to host the server on.",
        default_value = "127.0.0.1"
    )]
    ip: String,
}

// Pretty deprecated but might be useful later.
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

// This is the actual game handler. We should not return
// until the player has disconnected.
async fn game_handler(
    mut reader: TcpStream,
    mut writer: TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {

    
    
}

async fn accept_handler(
    mut reader: TcpStream,
    mut writer: TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    /*if let Err(e) = copy(reader, writer).await {
        println!("Found a stream error! {}", e);
    }*/
    let res = read_string(&mut reader).await?.1;
    println!("Server got info: {}", res);
    if let Some(s) = res.split(';').next() {
        if s == TcpStream::version_string() {
            println!("Successful connection!");
            write_string(&mut writer, ProtocolError::successful_connection()).await?;
        } else {
            println!("Wrong version!");
            write_string(&mut writer, ProtocolError::incorrect_version()).await?;
        }
    } else {
        println!("Unsuccessful connection!");
        write_string(&mut writer, ProtocolError::unknown_error()).await?;
    }

    //write_string(&mut writer, String::from("Goodbye.")).await
    game_handler(reader, writer).await
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
                if let Err(e) = accept_handler(stream.clone(), stream).await {
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
