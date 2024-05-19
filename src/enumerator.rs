use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use crate::device::{Class, Device};

pub struct Enumerator {
    backlights: BTreeMap<String, Device>,
    leds: BTreeMap<String, Device>,
}

impl Enumerator {
    pub fn new() -> Self {
        Enumerator {
            backlights: BTreeMap::new(),
            leds: BTreeMap::new(),
        }
    }

    pub fn scan(&mut self) -> Result<()> {
        self.backlights = self.scan_backlights()?;
        self.leds = self.scan_leds()?;

        Ok(())
    }

    fn scan_device(&self, class: Class, name: &str) -> Result<Device> {
        let path = PathBuf::from(format!("/sys/class/{}/{}", class, name));

        if !path.exists() {
            anyhow::bail!("Device not found: {}/{}", class, name);
        }

        if let Err(e) = fs::metadata(path.join("brightness")) {
            anyhow::bail!("Failed to read brightness for {}/{}: {e}", class, name);
        }
        if let Err(e) = fs::metadata(path.join("max_brightness")) {
            anyhow::bail!("Failed to read max_brightness for {}/{}: {e}", class, name);
        }

        Ok(Device {
            class,
            name: name.to_string(),
            real_name: None,
        })
    }

    fn scan_backlights(&self) -> Result<BTreeMap<String, Device>> {
        let paths =
            fs::read_dir("/sys/class/backlight").context("Failed to read /sys/class/backlight")?;
        let mut best_value: u32 = 0;
        let mut backlights = BTreeMap::new();

        for path in paths {
            let path = match path {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("Failed to read path: {e}");
                    continue;
                }
            };
            let Ok(name) = path.file_name().into_string() else {
                eprintln!("Failed to parse name for {:?}", path);
                continue;
            };

            let device = self.scan_device(Class::Backlight, &name)?;
            let max_brightness = device.max_brightness()?;

            // If the current backlight has a higher max brightness than the current best,
            // set it as the best (this is used for the "auto" target)
            if max_brightness > best_value {
                best_value = max_brightness;
                backlights.insert(
                    "auto".to_string(),
                    Device {
                        class: Class::Backlight,
                        name: "auto".to_string(),
                        real_name: Some(name.clone()),
                    },
                );
            }

            backlights.insert(name, device);
        }

        Ok(backlights)
    }

    fn scan_leds(&self) -> Result<BTreeMap<String, Device>> {
        let paths = fs::read_dir("/sys/class/leds").context("Failed to read /sys/class/leds")?;
        let mut leds = BTreeMap::new();

        for path in paths {
            let path = match path {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("Failed to read path: {e}");
                    continue;
                }
            };

            let name = match path.file_name().into_string() {
                Ok(name) => name,
                Err(e) => {
                    eprintln!("Failed to convert path to string: {e:?}");
                    continue;
                }
            };

            let device = self.scan_device(Class::Leds, &name)?;

            leds.insert(name, device);
        }

        Ok(leds)
    }

    pub fn get_default_device(&self) -> Result<Device> {
        let backlights = self.scan_backlights()?;

        if let Some(device) = backlights.get("auto") {
            return Ok(device.clone());
        }

        if let Some(device) = backlights.values().next() {
            return Ok(device.clone());
        }

        anyhow::bail!("No backlight devices found");
    }

    pub fn get_named_device(&self, name: &str) -> Result<Device> {
        let (class, name) = if name.contains('/') {
            let (class, name) = name.split_once('/').context("Invalid device name")?;
            (class, name)
        } else {
            ("backlight", name)
        };

        if name == "auto" {
            if class != "backlight" {
                anyhow::bail!("Invalid device name 'auto' for class '{class}'");
            }

            let backlights = self.scan_backlights()?;
            backlights
                .get("auto")
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("No auto device found"))
        } else {
            let device = match class {
                "backlight" => self.scan_device(Class::Backlight, name)?,
                "leds" => self.scan_device(Class::Leds, name)?,
                _ => {
                    anyhow::bail!("Unknown class: {class}");
                }
            };

            Ok(device)
        }
    }

    pub fn list(&self) -> Vec<&Device> {
        self.backlights.values().chain(self.leds.values()).collect()
    }
}

pub fn parse_device(device: &str) -> Result<Device> {
    let (class, name) = if device.contains('/') {
        let (class, name) = device.split_once('/').context("Invalid device name")?;
        (class, name)
    } else {
        ("backlight", device)
    };

    if name == "auto" {
        if class != "backlight" {
            anyhow::bail!("Invalid device name 'auto' for class '{class}'");
        }

        let mut enumerator = Enumerator::new();
        enumerator.scan()?;
        enumerator
            .get_default_device()
            .context("Failed to get default device")
    } else {
        let mut enumerator = Enumerator::new();
        enumerator.scan()?;
        enumerator
            .get_named_device(device)
            .context("Failed to get named device")
    }
}
