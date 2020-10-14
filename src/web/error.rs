pub mod cg {
    use std::{error, fmt};

    #[derive(Debug)]
    pub struct ProtocolError {
        err: String,
    }

    impl fmt::Display for ProtocolError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Protocol Error: {}", self.err)
        }
    }

    impl error::Error for ProtocolError {
        fn description(&self) -> &str {
            self.err.as_str()
        }
    }

    impl ProtocolError {
        #[allow(dead_code)]
        fn from(s: String) -> ProtocolError {
            ProtocolError { err: s }
        }
    }

    pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;
}
