use std::error;
use std::fmt;
use std::fmt::Formatter;

pub type IOResult<T> = Result<T, Error>;

#[derive(Debug, Clone)]
pub enum ErrorKind {
	FileIO,
	DataFormat
}

#[derive(Debug, Clone)]
pub struct Error(ErrorKind, String);

impl Error {
	pub fn new(kind: ErrorKind, message: String) -> Self {
		Self(kind, message)
	}
}

impl error::Error for Error {}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", self.1)
	}
}

impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Self { Self::new(ErrorKind::FileIO, e.to_string())}
}

impl From<bincode::Error> for Error {
	fn from(e: bincode::Error) -> Self { Self::new(ErrorKind::DataFormat, e.to_string())}
}

pub fn df_error(message: &str) -> Error {
	Error::new(ErrorKind::DataFormat, message.to_string())
}