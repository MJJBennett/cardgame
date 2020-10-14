use async_std::{net::TcpStream, prelude::*};

#[derive(Debug)]
struct WebStringError {
    err: String,
}

impl std::fmt::Display for WebStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "WebStringError: {}", self.err)
    }
}

impl std::error::Error for WebStringError {
    fn description(&self) -> &str {
        self.err.as_str()
    }
}

impl WebStringError {
    #[allow(dead_code)]
    fn from(s: String) -> WebStringError {
        WebStringError { err: s }
    }
}

async fn read_header(stream: &mut TcpStream) -> Result<usize, Box<dyn std::error::Error>> {
    // This method contains scary ASCII checks
    // Stay away!
    let mut buf = [0u8];
    let mut res = String::with_capacity(4);
    loop {
        let n = stream.read(&mut buf).await?;
        assert!(n == 1);
        res.push(match buf[0] {
            // eww
            i if (i < 58u8 && i > 47u8) => i as char,
            124u8 => break,
            e => {
                return Err(Box::new(WebStringError::from(format!(
                    "read_header: Found incorrect ASCII when reading: {}",
                    e
                ))))
            }
        });
    }

    println!("read_header: Parsing length.");
    let len = res.parse::<usize>()?;

    Ok(len)
}

/* Web client functionality (an asynchronous library)
 *
 */

pub async fn write_string(
    stream: &mut TcpStream,
    string: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let capacity = string.len() + 1 + 8;
    println!(
        "write_string: Using capacity {} for string of length {}.",
        capacity,
        string.len()
    );
    let mut s = String::with_capacity(capacity);
    s.push_str(&(string.len().to_string()));
    s.push('|');
    s.push_str(&string);
    println!(
        "write_string: Writing {} bytes after numeric: {:?}",
        string.len(),
        s.as_bytes()
    );
    stream.write_all(s.as_bytes()).await?;
    Ok(())
}

pub async fn read_string(
    stream: &mut TcpStream,
) -> Result<(usize, String), Box<dyn std::error::Error>> {
    // this should use take(16) or similar TODO to prevent hanging
    println!("read_string: Reading size.");
    let len = read_header(stream).await?;
    let mut buf = String::with_capacity(len);

    println!("read_string: Taking {} to string.", len as u64);
    stream.take(len as u64).read_to_string(&mut buf).await?;
    println!("read_string: Got result as '{}'", buf);

    Ok((len, buf))
}
