# pirate-midi-rs



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