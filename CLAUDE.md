# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an embedded Rust project targeting the Raspberry Pi Pico (RP2040) microcontroller, built using the Embassy async framework. The project implements a 4-way traffic light controller with proper phase transitions and safety delays.

## Technology Stack

- **Target Hardware**: Raspberry Pi Pico (RP2040 - ARM Cortex-M0+)
- **Framework**: Embassy 0.9.x (async embedded framework)
- **Toolchain**: Rust nightly (required for `impl_trait_in_assoc_type` feature)
- **Logging**: defmt 1.0 (efficient logging for embedded systems via RTT)
- **Panic Handler**: panic-probe 0.3 (defmt-based panic handling)

## Current Dependencies

```toml
embassy-rp = { version = "0.9", features = ["defmt", "time-driver", "critical-section-impl", "rp2040"] }
embassy-executor = { version = "0.9", features = ["nightly", "arch-cortex-m", "executor-thread"] }
embassy-time = { version = "0.5", features = ["defmt", "defmt-timestamp-uptime", "tick-hz-1_000_000"] }
defmt = "1.0"
defmt-rtt = "1.0"
panic-probe = { version = "1.0", features = ["print-defmt"] }
cortex-m-rt = "0.7"
```

**Important Notes:**
- `rp2040` feature is required in embassy-rp 0.9+
- `integrated-timers` feature was removed in embassy-executor 0.9+
- Nightly Rust is mandatory for Embassy executor features

## Build and Development Commands

### Prerequisites
Install required tools:
```bash
# Ensure nightly toolchain is installed (automatic via rust-toolchain.toml)
rustup target add thumbv6m-none-eabi --toolchain nightly
cargo install probe-rs-tools
```

### Build
```bash
cargo build --release
```

### Flash to Device
```bash
cargo run --release
```

### Debug Build
```bash
cargo build
```

## Architecture Notes

### Embassy Framework (v0.9)
- Uses Embassy's async executor for concurrent task management
- Time driver configured with 1MHz tick rate for precise timing
- Critical section implementation provided by embassy-rp
- Simplified API: `embassy_rp::init(Default::default())` for initialization

### GPIO Pin Naming
- Pins are accessed as `PIN_0`, `PIN_1`, `PIN_2`, etc. (not `GPIO0`, `GPIO1`)
- `Output<'_>` type no longer requires generic pin type parameter
- Example: `Output::new(p.PIN_25, Level::Low)`

### Timer API
- Modern API: `Timer::after_secs(10).await`
- Replaces: `Timer::after(Duration::from_secs(10)).await`

### Memory Model
- Cortex-M runtime (cortex-m-rt) handles startup and vector table
- No heap allocator by default (typical for embedded systems)
- Stack-based execution model with async futures

### Logging and Debugging
- defmt logging framework with RTT (Real-Time Transfer) transport
- Timestamps based on uptime counter
- Panic messages sent via defmt for cleaner debugging

## Important Constraints

- **Nightly Rust Required**: Uses `#![feature(impl_trait_in_assoc_type)]`
- **No Standard Library**: This is a `no_std` embedded environment
- **Resource Constraints**: RP2040 has 264KB SRAM and 2MB flash
- **Single-Core Focus**: Embassy-rp executor configured for single-threaded operation

## Project Structure

- `src/main.rs` - Traffic light controller with 4-way intersection logic
- `.cargo/config.toml` - Target configuration (thumbv6m-none-eabi) and probe-rs runner
- `rust-toolchain.toml` - Specifies nightly Rust channel
- `Cargo.toml` - Dependencies and project metadata

## Traffic Light Implementation

The project controls 12 GPIO pins (4 directions × 3 lights):
- **North**: PIN_0 (red), PIN_1 (yellow), PIN_2 (green)
- **East**: PIN_3 (red), PIN_4 (yellow), PIN_5 (green)
- **South**: PIN_6 (red), PIN_7 (yellow), PIN_8 (green)
- **West**: PIN_9 (red), PIN_10 (yellow), PIN_11 (green)

Cycle: North-South green (10s) → yellow (3s) → all red (1s) → East-West green (10s) → yellow (3s) → all red (1s) → repeat
