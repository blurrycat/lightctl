use std::fmt::Display;

use anyhow::Result;
use logind_zbus::session::SessionProxyBlocking;
use zbus::blocking::Connection;

use crate::util::read_u32_from_file;

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
        let connection = Connection::system()?;
        let session = SessionProxyBlocking::builder(&connection)
            .destination("org.freedesktop.login1")?
            .path("/org/freedesktop/login1/session/auto")?
            .interface("org.freedesktop.login1.Session")?
            .build()?;

        let name = self.real_name.as_ref().unwrap_or(&self.name);

        session.set_brightness(&self.class.to_string(), name, value)?;

        Ok(())
    }
}
