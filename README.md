# lightctl

A simple backlight (and LEDs) control utility. Uses the DBus interface provided by `(e)logind` to control brightness as a normal user.

## Usage
```sh
$ lightctl help
Usage: lightctl [OPTIONS] [COMMAND]

Commands:
  list    List available devices
  query   Query device info
  set     Set device brightness level
  status  Get device status
  toggle  Toggle device backlight
  help    Print this message or the help of the given subcommand(s)

Options:
  -d, --device <DEVICE>  
  -h, --help             Print help
```

### Examples
```sh
# List available devices
lightctl list

# Get device status
lightctl status

# Query device info
lightctl -d backlight/amdgpu_bl0 query max-brightness
lightctl -d backlight/amdgpu_bl0 query brightness
lightctl query default-device # Special case to return the default device name

# Set device brightness level
lightctl set 50% # Set to 50%
lightctl set +10% # Increase by 10%
lightctl set -10% # Decrease by 10%
lightctl set 255 # Set raw value to 255
lightctl set -50 # Decrease raw value by 50

# Toggle device backlight
# This will alternate between 0% and 100% brightness
lightctl -d leds/tpacpi::kbd_backlight toggle
```

## Installation

Install the binary directly through cargo:
```sh
cargo install --git https://github.com/blurrycat/lightctl
```

Alternatively, you can grab a pre-built binary from the [releases](https://github.com/blurrycat/lightctl/releases) page, and place it somewhere in your `$PATH`.

## Building

Pre-requisites:
* `just` (optional)
* `cargo`

```sh
just
# or
cargo build
```

Other `just` recipes are available:
```sh
$ just --list
Available recipes:
    build
    build-release
    build-release-static
    build-static
    default
    install
    run *ARGS
```