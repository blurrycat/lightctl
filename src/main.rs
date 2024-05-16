use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

use enumerator::Enumerator;
use value::BrightnessValue;

mod device;
mod enumerator;
mod util;
mod value;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    device: Option<String>,
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum GetParam {
    Brightness,
    MaxBrightness,
    DefaultDevice,
}

impl std::fmt::Display for GetParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetParam::Brightness => write!(f, "brightness"),
            GetParam::MaxBrightness => write!(f, "max-brightness"),
            GetParam::DefaultDevice => write!(f, "default-device"),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "List available devices")]
    List,
    #[clap(about = "Query device info")]
    Query {
        #[clap(help = "Get specific parameter value. Returns brightness when omitted.")]
        param: Option<GetParam>,
    },
    #[clap(about = "Set device brightness level")]
    Set {
        #[clap(
            allow_hyphen_values = true,
            help = "Value can be a raw value, percentage, relative value or relative percentage."
        )]
        value: String,
    },
    #[clap(about = "Get device status")]
    Status,
    #[clap(about = "Toggle device backlight")]
    Toggle,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut enumerator = Enumerator::new();

    let device = match args.device {
        Some(device) => enumerator.get_named_device(&device)?,
        None => enumerator.get_default_device()?,
    };

    match args.command {
        Some(Commands::List) => {
            enumerator.scan()?;
            println!("{}", "Available devices:".bold());
            for dev in enumerator.list() {
                println!("  {}", dev.to_string().green());
            }
        }
        Some(Commands::Query { param }) => match param {
            None | Some(GetParam::Brightness) => {
                let brightness = device.brightness()?;
                println!("{}", brightness);
            }
            Some(GetParam::MaxBrightness) => {
                let max_brightness = device.max_brightness()?;
                println!("{}", max_brightness);
            }
            Some(GetParam::DefaultDevice) => {
                let default_device = enumerator.get_default_device()?;
                println!("{}", default_device);
            }
        },
        Some(Commands::Set { value }) => {
            let value: BrightnessValue = value.parse()?;
            let value = match value {
                BrightnessValue::Absolute(value) => {
                    let max = device.max_brightness()?;
                    value.min(max).max(0)
                }
                BrightnessValue::Relative(value) => {
                    let current = device.brightness()?;
                    let max = device.max_brightness()?;
                    let new = (current as i32 + value) as u32;

                    new.min(max).max(0)
                }
                BrightnessValue::Percent(value) => {
                    let max = device.max_brightness()?;
                    let new = (value as f64 / 100.0 * max as f64) as u32;

                    new.min(max).max(0)
                }
                BrightnessValue::RelativePercent(value) => {
                    let current = device.brightness()?;
                    let max = device.max_brightness()?;
                    let new = (current as i32 + (value as f64 / 100.0 * max as f64) as i32) as u32;

                    new.min(max).max(0)
                }
            };
            device.set_brightness(value)?;
        }
        Some(Commands::Toggle) => {
            let current = device.brightness()?;
            let max = device.max_brightness()?;
            // let new = (current + max).min(max).max(0) - current;
            let new = if current != max { max } else { 0 };
            device.set_brightness(new)?;
        }
        None | Some(Commands::Status) => {
            let current = device.brightness()?;
            let max = device.max_brightness()?;
            let percentage = format!("{:.2}%", current as f64 / max as f64 * 100.0);

            println!("{}", "Device status".bold());
            if device.name.as_str() == "auto" {
                println!(
                    "Device: {} ({})",
                    device.to_string().green(),
                    device.real_name.as_ref().unwrap().green()
                );
            } else {
                println!("Device: {}", device.to_string().green());
            }
            println!(
                "Current brightness: {} ({})",
                current.to_string().green(),
                percentage.green()
            );
            println!("Max brightness: {}", max.to_string().green());
        }
    }

    Ok(())
}
