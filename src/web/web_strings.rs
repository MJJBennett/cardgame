use async_std::{net::TcpStream, prelude::*};

/** WebStringError
 *
 * An error class currently used for errors encountered
 * while parsing strings from the TcpStream.
 */
#[derive(Debug)]
struct WebStringError {
    err: String,
}

// This trait implementation allows the error to be printed.
impl std::fmt::Display for WebStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "WebStringError: {}", self.err)
    }
}

// This trait implementation essentially makes us an Error subclass.
impl std::error::Error for WebStringError {
    fn description(&self) -> &str {
        self.err.as_str()
    }
}

// Allow for easily creating the error from a string.
impl WebStringError {
    fn from(s: String) -> WebStringError {
        WebStringError { err: s }
    }
}

// Advances the stream until it only contains text
// (In other words, removes the header from the message)
// Then returns the parsed size of the text.
async fn read_header(stream: &mut TcpStream) -> Result<usize, Box<dyn std::error::Error>> {
    // This method contains scary ASCII checks
    // Stay away!
    let mut buf = [0u8]; // Array buffer.
    let mut res = String::with_capacity(4); // Output string.
    loop {
        // Read exactly one byte. TcpStream knows
        // to read one byte because the buffer is
        // an array of size one, and type u8.
        let _ = stream.read(&mut buf).await?;
        res.push(match buf[0] {
            // eww
            i if (i < 58u8/*9*/ && i > 47u8/*0*/) => i as char,
            124u8/*|*/ => break,
            l if (l == 0u8 && res.len() == 0) => {
                return Err(Box::new(WebStringError::from(String::from(
                    "read_header: Found disconnection.",
                ))))
            }
            e/*Any other u8 not matched above*/ => {
                return Err(Box::new(WebStringError::from(format!(
                    "read_header: Found incorrect ASCII when reading: {}",
                    e
                ))))
            }
        });
    }

    /* Essentially, say for string 2|hi (valid), we read
     * characters until we read | - so the stream ends up
     * as 'hi'. Then the characters excluding | are converted
     * to an integer here:
     */
    println!("read_header: Parsing length.");
    let len = res.parse::<usize>()?;

    Ok(len)
}

/* Web client functionality (an asynchronous library)
 *
 * Provides read_string and write_string as public functions.
 *
 * write_string is used for writing a string to the stream,
 * and read_string is used for reading a string from the stream.
 */

// Returns nothing on success, or an error if encountered.
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

// Returns a tuple on success - (size of string, string).
// Generally errors if read_header errors, on an incorrect
// header or disconnection.
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
