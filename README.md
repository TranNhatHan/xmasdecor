# ESP32-C3 Christmas LED Matrix

A Christmas-themed LED matrix animation built with **Rust**, **ESP32-C3**, **esp-hal**, and **Embassy**.

The project drives a 14×18 WS2812 RGB LED matrix (252 LEDs), displaying an animated Christmas tree, twinkling star, colourful ornaments, falling snow, and Christmas presents.

The project can be developed and tested on real ESP32-C3 hardware or simulated using [Wokwi](https://wokwi.com/).

## Hardware

| Component | Description |
|---|---|
| Microcontroller | ESP32-C3 |
| LED Matrix | WS2812 RGB |
| Matrix Size | 14 × 18 |
| Total LEDs | 252 |
| LED Data Pin | GPIO10 |

### Hardware Connections

| ESP32-C3 | WS2812 Matrix |
|---|---|
| GPIO10 | DIN |
| 5V external power supply | 5V |
| GND | GND |

**Important:** Use an adequately rated external power supply for the LED matrix. Do not power all 252 LEDs directly from the ESP32-C3 board. Connect the power supply ground and ESP32-C3 ground together.

## Software and Technologies

- [Rust](https://www.rust-lang.org/)
- [esp-hal](https://github.com/esp-rs/esp-hal) – ESP32 hardware abstraction layer
- [Embassy](https://github.com/embassy-rs/embassy) – Asynchronous embedded framework
- [esp-rs](https://github.com/esp-rs) – Rust ecosystem for Espressif chips
- [WS2812 driver](https://crates.io/crates/rustyfarian-esp-hal-ws2812) – LED matrix control
- [Wokwi](https://wokwi.com/) – Embedded hardware simulator

### Architecture

The project separates application logic from hardware-specific implementation.

- **main.rs** – Initializes the ESP32-C3 and Embassy runtime.
- **config/** – Contains shared configuration such as matrix dimensions, brightness, and animation interval.
- **app/christmas.rs** – Implements the Christmas animation and drawing logic.
- **drivers/ws2812.rs** – Initializes the ESP32-C3 RMT peripheral and WS2812 driver.

## Getting Started

### Prerequisites

Install Rust and the ESP32 Rust toolchain.

See the [esp-rs documentation](https://docs.espressif.com/projects/rust/book/) for setup instructions.

### Clone the repository

```bash
git clone https://github.com/TranNhatHan/xmasdecor.git

cd xmasdecor
```

### Build

```bash
cargo check
```

### Flash the ESP32-C3

```bash
cargo run
```

The exact flashing command depends on your configured Cargo runner and ESP32-C3 setup.

## Wokwi Simulation

This project can also be explored using [Wokwi](https://wokwi.com/), an electronics simulator.

Wokwi supports ESP32-C3 simulation and addressable LED components, making it useful for experimenting with the animation without physical hardware.

### 1. Install Wokwi

Install the Wokwi extension for Visual Studio Code:

https://marketplace.visualstudio.com/items?itemName=Wokwi.wokwi-vscode

Once installed, open your project folder in VS Code and make sure the Wokwi extension is enabled for the workspace.

### 2. Build the firmware

Build your ESP32-C3 firmware by running the following command from the project's root directory:

```bash
cargo build
```

### 3. Run the simulation

-Press 'Ctrl + Shift + P' to open the Command Palette.
-Search for and select 'Wokwi: Start Simulator'.
-The Christmas animation should now appear on the simulated LED component.


## Configuration

You can customize the animation in `src/bin/config/mod.rs`.

| Setting | Description |
|---|---|
| `WIDTH` | LED matrix width |
| `HEIGHT` | LED matrix height |
| `NUM_SNOWFLAKES` | Number of falling snowflakes |
| `BRIGHTNESS` | Overall LED brightness |
| `FRAME_INTERVAL_MS` | Animation frame interval |

## Demo
<img width="561" height="764" alt="Screenshot From 2026-11-25 23-11-59" src="https://github.com/user-attachments/assets/9490b3b3-b44a-4cd0-8b78-5270c6f6a551" />

---
Made with ❤️, Rust, and a little Christmas spirit! 🎄
````
