use pirate_midi_rs::{Command, PirateMIDIDevice};

fn main() {
    match PirateMIDIDevice::new().send(Command::Check) {
        Ok(response) => println!("success: {:?}", response),
        Err(err) => println!("error getting response: {:?}", err),
    }
}
