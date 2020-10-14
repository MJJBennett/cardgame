// This module contains the necessary protocol code, 
// so that it can be easily shared between the client 
// and the server.
// The protocols are implemented as traits on TcpStream.
use async_std::net::TcpStream;
use async_trait::async_trait;

use super::web_strings::*;
use super::error::*;

#[async_trait]
pub trait Protocol {
    async fn initialize(&mut self) -> cg::Result;
    async fn poll_server(&mut self) -> cg::Result;

    fn version_string() -> String { String::from(env!("CARGO_PKG_VERSION")) }

    fn initialize_string() -> String { format!("{};{}", Self::version_string(), "NOT_IMPLEMENTED") }
    fn poll_string() -> String { String::from("POLL_STRING") }
}

#[async_trait]
impl Protocol for TcpStream {
    async fn initialize(&mut self) -> cg::Result {
        write_string(self, Self::initialize_string()).await?;
        Ok(())
    }

    async fn poll_server(&mut self) -> cg::Result {
        write_string(self, Self::poll_string()).await?;
        Ok(())
    }
}
