#[derive(Debug)]
pub enum Error {
    TryFromConversion,
    IOError(std::io::Error),
    SysFsBatteryItem,
    SysFsBatteryItemPath,
    SysFsBatteryPath,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TryFromConversion => write!(f, "A conversion error occurred."),
            Error::IOError(e) => write!(f, "An IO error occurred: {}", e),
            Error::SysFsBatteryItem => write!(f, "The requested item is empty."),
            Error::SysFsBatteryItemPath => write!(f, "The requested item does not exist."),
            Error::SysFsBatteryPath => {
                write!(f, "/sys/class/power_supply/BAT1/ is not a directory.")
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
