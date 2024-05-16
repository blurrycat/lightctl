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
