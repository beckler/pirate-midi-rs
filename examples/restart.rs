use pirate_midi_rs::{Command, ControlArgs, PirateMIDIDevice};

fn main() {
    match PirateMIDIDevice::new().send(Command::Control(ControlArgs::DeviceRestart)) {
        Ok(response) => println!("success: {:?}", response),
        Err(err) => println!("error getting response: {:?}", err),
    }
}
