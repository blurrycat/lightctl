use std::fmt::Display;

use anyhow::Result;

use crate::{dbus, util::read_u32_from_file};

#[derive(Clone)]
pub enum Class {
    Backlight,
    Leds,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::Backlight => write!(f, "backlight"),
            Class::Leds => write!(f, "leds"),
        }
    }
}

#[derive(Clone)]
pub struct Device {
    pub class: Class,
    pub name: String,
    pub real_name: Option<String>,
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.class, self.name)
    }
}

impl Device {
    pub fn brightness(&self) -> Result<u32> {
        let name = self.real_name.as_ref().unwrap_or(&self.name);
        read_u32_from_file(&format!("/sys/class/{}/{}/brightness", self.class, name))
    }

    pub fn max_brightness(&self) -> Result<u32> {
        let name = self.real_name.as_ref().unwrap_or(&self.name);
        read_u32_from_file(&format!(
            "/sys/class/{}/{}/max_brightness",
            self.class, name
        ))
    }

    pub fn set_brightness(&self, value: u32) -> Result<()> {
        let name = self.real_name.as_ref().unwrap_or(&self.name);

        dbus::set_brightness(&self.class.to_string(), name, value)?;

        Ok(())
    }
}
