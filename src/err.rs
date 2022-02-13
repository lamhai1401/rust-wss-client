use snafu::*;
// use std::fmt;
use tokio_tungstenite::tungstenite::Error as TungErr;

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[snafu(display(
        // "Undefined value length of element tagged {} at position {}",
        // tag,
        // position
        "Connecting to Wss Err {}",

        details
    ))]
    #[non_exhaustive]
    WssConnectionErr { details: String },
}

impl From<TungErr> for Error {
    fn from(err: TungErr) -> Error {
        Error::WssConnectionErr {
            details: err.to_string(),
        }
    }
}

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "There is an error: {}", self.0)
//     }
// }
