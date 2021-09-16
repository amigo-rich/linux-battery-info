use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    /// A &str to enum conversion error.
    TryFromConversion,
    /// A wrapper around a std::io::Error.
    IOError(std::io::Error),
    /// The content of a requested file is empty.
    SysFsBatteryItem,
    /// A requested file is either not a file, or does not exist.
    SysFsBatteryItemPath(PathBuf),
    /// The item (e.g. capacity, capacity level etc. is unknown to the program.
    SysFsBatteryItemUnknown(String),
    /// The requested directory is either not a director or does not exist.
    SysFsBatteryPath(PathBuf),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TryFromConversion => write!(f, "A conversion error occurred."),
            Error::IOError(e) => write!(f, "An IO error occurred: {}", e),
            Error::SysFsBatteryItem => write!(f, "The requested item is empty."),
            Error::SysFsBatteryItemPath(pb) => write!(f, "{} is not a file.", pb.display()),
	    Error::SysFsBatteryItemUnknown(i) => write!(f, "{} is an unknown item.", i),
            Error::SysFsBatteryPath(pb) => {
                write!(f, "{} is not a directory.", pb.display())
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}
