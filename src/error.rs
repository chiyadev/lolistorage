use std::{error, fmt};

#[derive(Debug)]
pub enum LoliError {
    // Reqwest(reqwest::Error),
    Unknown,
}

impl error::Error for LoliError {}

// impl From<reqwest::Error> for LoliError {
//     fn from(err: reqwest::Error) -> LoliError {
//         LoliError::Reqwest(err)
//     }
// }

impl fmt::Display for LoliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // LoliError::Reqwest(err) => err.fmt(f),
            LoliError::Unknown => write!(f, "unknown error"),
        }
    }
}
