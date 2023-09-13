use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CmdError(pub String);
impl Error for CmdError {}
impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}
