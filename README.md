# pirate-midi-rs

[![Documentation](https://docs.rs/pirate-midi-rs/badge.svg)](https://docs.rs/pirate-midi-rs/latest/pirate_midi_rs/)
[![Crates.io](https://img.shields.io/crates/v/pirate-midi-rs.svg)](https://crates.io/crates/pirate-midi-rs)
[![License](https://img.shields.io/crates/l/pirate-midi-rs.svg)](https://github.com/beckler/pirate-midi-rs/blob/main/LICENSE)
[![Test Status](https://github.com/beckler/pirate-midi-rs/workflows/release-please/badge.svg?event=push)](https://github.com/beckler/pirate-midi-rs/actions)

## Description

Rust library for sending serial commands to Pirate MIDI Bridge devices

## Usage

Make sure your bridge device is connected via USB before running a command.

```rust
    // create our test device
    let device = PirateMIDIDevice::new();

    // get device details
    let response = device.send(Command::Check).unwrap(); // use a `match` instead of `.unwrap()`

    // enter the bootloader
    // the device will detach and then reattach in bootloader mode, and the serial port will no longer be available.
    // will return `ok`, but any subsequent commands will throw an error
    match PirateMIDIDevice::new().send(Command::Control(ControlArgs::EnterBootloader)) {
        Ok(_) => Ok(()),
        Err(err) => panic!("unable to enter bootloader: {}", err),
    }
```
