[![Rust CI](https://github.com/simon0302010/signal-locate/actions/workflows/rust.yml/badge.svg)](https://github.com/simon0302010/signal-locate/actions/workflows/rust.yml)
![Hackatime](https://hackatime-badge.hackclub.com/U08HC7N4JJW/signal-locate)
![Crates.io Total Downloads](https://img.shields.io/crates/d/signal-locate)
![Crates.io Version](https://img.shields.io/crates/v/signal-locate)

# Signal Locate

A tool to create a heatmap of wifi signal strength.

## Demo

https://github.com/user-attachments/assets/79be7a2d-d2c9-4647-ad41-b8cca2ac6b1b

## Requirements

- A WiFi adapter
- Rust (when building from source)

## Compatibility Notes

- macOS support is currently broken.

## Installation

You can install Signal Locate using Cargo:

```bash
cargo install signal-locate
```

You can also download precompiled binaries from [Actions](https://github.com/simon0302010/signal-locate/actions/workflows/rust.yml).

## Usage

To use Signal Locate, run the following command:

```bash
sudo -E signal-locate
```
> Run the precompiled binary if you use that.
> Root privileges are required to scan for WiFi networks.
> macOS and Windows may not require elevated priveleges.

- After starting, open a room plan of your house (currently, only one floor is supported).
- Select the WiFi network you want to map from the dropdown on the right.
- Walk around your house and click your current position on the map to measure signal strength.
- Collect at least 10 points for a good map.
- When finished, click the **Create Heatmap** button to generate the heatmap.

## License

This project is licensed under the GNU General Public License Version 3. See the [LICENSE](LICENSE) file for details.
