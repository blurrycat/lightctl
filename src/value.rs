use anyhow::Result;

pub enum BrightnessValue {
    Absolute(u32),
    Percent(u32),
    Relative(i32),
    RelativePercent(i32),
}

impl std::str::FromStr for BrightnessValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.ends_with('%') {
            let value = s.trim_end_matches('%').parse()?;

            if s.starts_with('+') || s.starts_with('-') {
                Ok(BrightnessValue::RelativePercent(value))
            } else {
                Ok(BrightnessValue::Percent(value as u32))
            }
        } else if s.starts_with('+') || s.starts_with('-') {
            let value = s.parse()?;
            Ok(BrightnessValue::Relative(value))
        } else {
            let value = s.parse()?;
            Ok(BrightnessValue::Absolute(value))
        }
    }
}

impl BrightnessValue {
    pub fn calculate(&self, current: u32, max: u32) -> u32 {
        match self {
            BrightnessValue::Absolute(value) => (*value).min(max).max(0),
            BrightnessValue::Relative(value) => {
                let new = (current as i32 + value) as u32;
                new.min(max).max(0)
            }
            BrightnessValue::Percent(value) => {
                let new = (*value as f64 / 100.0 * max as f64) as u32;
                new.min(max).max(0)
            }
            BrightnessValue::RelativePercent(value) => {
                let new = (current as i32 + (*value as f64 / 100.0 * max as f64) as i32) as u32;
                new.min(max).max(0)
            }
        }
    }
}
