// use std::{error::Error, fmt::Display, result};

// pub type Result<T> = result::Result<T, MyError>;

// #[derive(Debug)]
// pub enum MyError {
//     IO(std::io::Error),
//     Http(reqwest::Error),
// }

// impl From<std::io::Error> for MyError {
//     fn from(e: std::io::Error) -> Self {
//         Self::IO(e)
//     }
// }

// impl From<reqwest::Error> for MyError {
//     fn from(e: reqwest::Error) -> Self {
//         Self::Http(e)
//     }
// }

// impl Display for MyError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl Error for MyError {}
