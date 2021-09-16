use crate::error::Error;
use std::convert::TryFrom;
use std::path::Path;

#[derive(Debug)]
pub enum PowerSupplyStatus {
    Unknown,
    Charging,
    Discharging,
    NotCharging,
    Full,
}

impl Default for PowerSupplyStatus {
    fn default() -> Self {
        PowerSupplyStatus::Unknown
    }
}

impl TryFrom<&str> for PowerSupplyStatus {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Error::TryFromConversion);
        }
        let status = match value.trim() {
            "Charging" => PowerSupplyStatus::Charging,
            "Discharging" => PowerSupplyStatus::Discharging,
            "Not_charging" => PowerSupplyStatus::NotCharging,
            "Full" => PowerSupplyStatus::Full,
            _ => PowerSupplyStatus::Unknown,
        };
        Ok(status)
    }
}

impl std::fmt::Display for PowerSupplyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            PowerSupplyStatus::Unknown => "Unknown",
            PowerSupplyStatus::Charging => "Charging",
            PowerSupplyStatus::Discharging => "Discharging",
            PowerSupplyStatus::NotCharging => "Not charging",
            PowerSupplyStatus::Full => "Full",
        };
        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub enum PowerSupplyCapacity {
    Unknown,
    Level(u8),
}

impl Default for PowerSupplyCapacity {
    fn default() -> Self {
        PowerSupplyCapacity::Unknown
    }
}

impl TryFrom<&str> for PowerSupplyCapacity {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Error::TryFromConversion);
        }
        let level: u8 = match value.trim().parse() {
            Ok(level) => {
                if level > 100 {
                    return Err(Error::TryFromConversion);
                }
                level
            }
            Err(_) => return Err(Error::TryFromConversion),
        };
        Ok(PowerSupplyCapacity::Level(level))
    }
}

impl std::fmt::Display for PowerSupplyCapacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let PowerSupplyCapacity::Level(level) = self {
            write!(f, "{}", level)
        } else {
            write!(f, "Unknown")
        }
    }
}

#[derive(Debug)]
pub enum PowerSupplyCapacityLevel {
    Unknown,
    Critical,
    Low,
    Normal,
    High,
    Full,
}

impl Default for PowerSupplyCapacityLevel {
    fn default() -> Self {
        PowerSupplyCapacityLevel::Unknown
    }
}

impl TryFrom<&str> for PowerSupplyCapacityLevel {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Error::TryFromConversion);
        }
        let capacity_level = match value.trim() {
            "Critical" => PowerSupplyCapacityLevel::Critical,
            "Low" => PowerSupplyCapacityLevel::Low,
            "Normal" => PowerSupplyCapacityLevel::Normal,
            "High" => PowerSupplyCapacityLevel::High,
            "Full" => PowerSupplyCapacityLevel::Full,
            _ => PowerSupplyCapacityLevel::Unknown,
        };
        Ok(capacity_level)
    }
}

impl std::fmt::Display for PowerSupplyCapacityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            PowerSupplyCapacityLevel::Unknown => "Unknown",
            PowerSupplyCapacityLevel::Critical => "Critical",
            PowerSupplyCapacityLevel::Low => "Low",
            PowerSupplyCapacityLevel::Normal => "Normal",
            PowerSupplyCapacityLevel::High => "High",
            PowerSupplyCapacityLevel::Full => "Full",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug)]
pub enum PowerSupplyManufacturer {
    Unknown,
    Manufacturer(String),
}

impl Default for PowerSupplyManufacturer {
    fn default() -> Self {
        PowerSupplyManufacturer::Unknown
    }
}

impl TryFrom<&str> for PowerSupplyManufacturer {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Error::TryFromConversion);
        }
        let manufacturer = String::from(value.trim());
        Ok(PowerSupplyManufacturer::Manufacturer(manufacturer))
    }
}

impl std::fmt::Display for PowerSupplyManufacturer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let PowerSupplyManufacturer::Manufacturer(name) = self {
            write!(f, "{}", name)
        } else {
            write!(f, "Unknown")
        }
    }
}

#[derive(Debug)]
pub enum PowerSupplyModelName {
    Unknown,
    ModelName(String),
}

impl Default for PowerSupplyModelName {
    fn default() -> Self {
        PowerSupplyModelName::Unknown
    }
}

impl TryFrom<&str> for PowerSupplyModelName {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Error::TryFromConversion);
        }
        let model_name = String::from(value.trim());
        Ok(PowerSupplyModelName::ModelName(model_name))
    }
}

impl std::fmt::Display for PowerSupplyModelName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let PowerSupplyModelName::ModelName(model_name) = self {
            write!(f, "{}", model_name)
        } else {
            write!(f, "Unknown")
        }
    }
}

#[derive(Debug)]
pub enum PowerSupplySerialNumber {
    Unknown,
    SerialNumber(String),
}

impl Default for PowerSupplySerialNumber {
    fn default() -> Self {
        PowerSupplySerialNumber::Unknown
    }
}

impl TryFrom<&str> for PowerSupplySerialNumber {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Error::TryFromConversion);
        }
        let serial_number = String::from(value.trim());
        Ok(PowerSupplySerialNumber::SerialNumber(serial_number))
    }
}

impl std::fmt::Display for PowerSupplySerialNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let PowerSupplySerialNumber::SerialNumber(serial_number) = self {
            write!(f, "{}", serial_number)
        } else {
            write!(f, "Unknown")
        }
    }
}

#[derive(Debug)]
pub struct Battery {
    capacity: PowerSupplyCapacity,
    status: PowerSupplyStatus,
    capacity_level: PowerSupplyCapacityLevel,
    manufacturer: PowerSupplyManufacturer,
    model_name: PowerSupplyModelName,
    serial_number: PowerSupplySerialNumber,
}

impl std::fmt::Display for Battery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} ({}) | {}% ({}) | {} ",
            self.manufacturer,
            self.model_name,
            self.serial_number,
            self.capacity,
            self.capacity_level,
            self.status,
        )
    }
}

impl Battery {
    pub fn new() -> Result<Battery, Error> {
        let mut capacity = PowerSupplyCapacity::default();
        let mut status = PowerSupplyStatus::default();
        let mut capacity_level = PowerSupplyCapacityLevel::default();
        let mut manufacturer = PowerSupplyManufacturer::default();
        let mut model_name = PowerSupplyModelName::default();
        let mut serial_number = PowerSupplySerialNumber::default();

        let sys_fs_battery_path = Path::new("/sys/class/power_supply/BAT1/");
        if !sys_fs_battery_path.is_dir() {
            return Err(Error::SysFsBatteryPath);
        }
        for item in &[
            "capacity",
            "capacity_level",
            "manufacturer",
            "model_name",
            "serial_number",
            "status",
        ] {
            let item_path = sys_fs_battery_path.join(Path::new(item));
            if !item_path.is_file() {
                return Err(Error::SysFsBatteryItemPath);
            }
            let item_content = std::fs::read_to_string(&item_path)?;
            if item_content.is_empty() {
                return Err(Error::SysFsBatteryItem);
            }
            match *item {
                "capacity" => {
                    capacity = PowerSupplyCapacity::try_from(item_content.as_str())?
                }
                "capacity_level" => {
                    capacity_level =
                        PowerSupplyCapacityLevel::try_from(item_content.as_str())?
                }
                "manufacturer" => {
                    manufacturer = PowerSupplyManufacturer::try_from(item_content.as_str())?
                }
                "model_name" => {
                    model_name = PowerSupplyModelName::try_from(item_content.as_str())?
                }
                "serial_number" => {
                    serial_number =
                        PowerSupplySerialNumber::try_from(item_content.as_str())?
                }
                "status" => status = PowerSupplyStatus::try_from(item_content.as_str())?,
                _ => return Err(Error::SysFsBatteryItem),
            }
        }
        Ok(Battery {
            capacity,
            capacity_level,
            manufacturer,
            model_name,
            serial_number,
            status,
        })
    }
}
