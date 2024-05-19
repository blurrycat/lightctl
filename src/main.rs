use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

use device::Device;
use enumerator::Enumerator;
use value::BrightnessValue;

mod dbus;
mod device;
mod enumerator;
mod util;
mod value;

#[derive(Parser)]
struct Cli {
    // #[clap(short, long)]
    // device: Option<String>,
    #[clap(
        short,
        long,
        value_parser = enumerator::parse_device,
        default_value = "backlight/auto"
    )]
    device: Device,
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

    match args.command {
        Some(Commands::List) => {
            let mut enumerator = Enumerator::new();
            enumerator.scan()?;
            println!("{}", "Available devices:".bold());
            for dev in enumerator.list() {
                println!("  {}", dev.to_string().green());
            }
        }
        Some(Commands::Query { param }) => match param {
            None | Some(GetParam::Brightness) => {
                let brightness = args.device.brightness()?;
                println!("{}", brightness);
            }
            Some(GetParam::MaxBrightness) => {
                let max_brightness = args.device.max_brightness()?;
                println!("{}", max_brightness);
            }
            Some(GetParam::DefaultDevice) => {
                let enumerator = Enumerator::new();
                let default_device = enumerator.get_default_device()?;
                println!("{}", default_device);
            }
        },
        Some(Commands::Set { value }) => {
            let value: BrightnessValue = value.parse()?;

            let current = args.device.brightness()?;
            let max = args.device.max_brightness()?;
            let new = value.calculate(current, max);

            args.device.set_brightness(new)?;
        }
        Some(Commands::Toggle) => {
            let current = args.device.brightness()?;
            let max = args.device.max_brightness()?;
            // let new = (current + max).min(max).max(0) - current;
            let new = if current != max { max } else { 0 };
            args.device.set_brightness(new)?;
        }
        None | Some(Commands::Status) => {
            let current = args.device.brightness()?;
            let max = args.device.max_brightness()?;
            let percentage = format!("{:.2}%", current as f64 / max as f64 * 100.0);

            println!("{}", "Device status".bold());
            if args.device.name.as_str() == "auto" {
                println!(
                    "Device: {} ({})",
                    args.device.to_string().green(),
                    args.device.real_name.as_ref().unwrap().green()
                );
            } else {
                println!("Device: {}", args.device.to_string().green());
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
