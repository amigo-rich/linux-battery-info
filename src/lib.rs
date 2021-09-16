pub mod battery;
use battery::Battery;
pub mod error;
use error::Error;

pub fn run() -> Result<Battery, Error> {
    Battery::new()
}
