use log::trace;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, io::ErrorKind, time::Duration};

pub use messages::*;
use serialport::{available_ports, SerialPort, SerialPortBuilder, SerialPortType};
use thiserror::Error;

pub mod messages;

pub const VENDOR_ID: u16 = 0x0483;
pub const PRODUCT_ID: u16 = 0x5740;
pub const USB_TIMEOUT: Duration = Duration::from_secs(1);
pub const USB_BAUD_RATE: u32 = 9600;

/// Arguments for Data Transmit Requests (DTXR)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DataTransmitRequestArgs {
    ProfileID(String),
    GlobalSettings,
    BankSettings(u8),
}

impl Display for DataTransmitRequestArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            DataTransmitRequestArgs::ProfileID(s) => format!("profileId,{s}"),
            DataTransmitRequestArgs::GlobalSettings => "globalSettings".to_string(),
            DataTransmitRequestArgs::BankSettings(x) => format!("bankSettings,{x}"),
        };
        write!(f, "{output}")
    }
}

/// Arguments for Data Requests (DREQ)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DataRequestArgs {
    GlobalSettings,
    BankSettings(i8),
}

impl Display for DataRequestArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            DataRequestArgs::GlobalSettings => "globalSettings".to_string(),
            DataRequestArgs::BankSettings(x) => format!("bankSettings,{x}"),
        };
        write!(f, "{output}")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ControlRequestArgs {
    command: Vec<ControlArgs>,
}

/// Arguments for Control Requests (CTRL)
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ControlArgs {
    BankUp,
    BankDown,
    GoToBank(i8),
    ToggleFootswitch(i8),
    DeviceRestart,
    EnterBootloader,
    FactoryReset,
}

impl ControlArgs {
    pub fn format(&self, backwards_compatable: bool) -> String {
        // support v1 formats
        if backwards_compatable {
            return format!("{}", self);
        }

        // v2 formats - don't love this, we should probably redo this at some point
        serde_json::to_string(&ControlRequestArgs {
            command: vec![self.clone()],
        })
        .unwrap()
    }
}

impl Display for ControlArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ControlArgs::BankUp => "bankUp".to_string(),
            ControlArgs::BankDown => "bankDown".to_string(),
            ControlArgs::GoToBank(x) => format!("goToBank,{x}"),
            ControlArgs::ToggleFootswitch(x) => format!("toggleFootswitch,{x}"),
            ControlArgs::DeviceRestart => "deviceRestart".to_string(),
            ControlArgs::EnterBootloader => "enterBootloader".to_string(),
            ControlArgs::FactoryReset => "factoryReset".to_string(),
        };
        write!(f, "{output}")
    }
}

/// Commands to send to device
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Command {
    /// Check Command (CHCK)
    /// This queries the device about what type of device it is, what firmware version is it running, and other essential information.
    Check,
    /// Control Command (CTRL)
    /// Provides control over basic device functions such as footswitch behaviour, bank navigation, and reset modes.
    Control(ControlArgs),
    /// Data Request Command (DREQ)
    /// Requested by the host application to the device. This prompts the device for global, bank, or configuration data.
    DataRequest(DataRequestArgs),
    /// Data Transmit Request (DTXR)
    /// Informs the device that the host appliccation wishes to transmit new data.
    DataTransmitRequest(DataTransmitRequestArgs),
    /// Reset Command (RSET)
    /// Resets the communication state of the device to exit a command state if required.
    Reset,
}

impl Command {
    pub fn format(&self, backwards_compatable: bool) -> Vec<String> {
        match self {
            Command::Check | Command::Reset => vec![format!("{self}")],
            Command::Control(args) => vec![format!("{self}"), args.format(backwards_compatable)],
            Command::DataRequest(args) => vec![format!("{self}"), format!("{args}")],
            Command::DataTransmitRequest(args) => vec![format!("{self}"), format!("{args}")],
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Command::Check => "CHCK",
            Command::Control(_) => "CTRL",
            Command::DataRequest(_) => "DREQ",
            Command::DataTransmitRequest(_) => "DTXR",
            Command::Reset => "RSET",
        };
        write!(f, "{output}")
    }
}

/// Represents a Pirate MIDI serial client
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PirateMIDIDevice {
    vid: u16,
    pid: u16,
    timeout: Duration,
    baud_rate: u32,
    builder: Option<SerialPortBuilder>,
}

impl Default for PirateMIDIDevice {
    fn default() -> Self {
        Self {
            vid: VENDOR_ID,
            pid: PRODUCT_ID,
            timeout: USB_TIMEOUT,
            baud_rate: USB_BAUD_RATE,
            builder: None,
        }
    }
}

impl PirateMIDIDevice {
    /// Creates a new serial client
    pub fn new() -> PirateMIDIDevice {
        Self::default()
    }

    /// By default we attempt to connect with devices with the constant VENDOR_ID
    /// However, this method exists for if this changes in the future.
    pub fn with_vendor_id(&self, vid: u16) -> PirateMIDIDevice {
        Self {
            vid,
            pid: self.pid,
            timeout: self.timeout,
            baud_rate: self.baud_rate,
            builder: self.builder.clone(),
        }
    }

    /// By default we attempt to connect with devices with the constant PRODUCT_ID
    /// However, this method exists for if this changes in the future.
    pub fn with_product_id(&self, pid: u16) -> PirateMIDIDevice {
        Self {
            vid: self.vid,
            pid,
            timeout: self.timeout,
            baud_rate: self.baud_rate,
            builder: self.builder.clone(),
        }
    }

    /// By default we attempt to connect with devices with the constant BAUD_RATE
    /// However, this method exists for if this changes in the future.
    pub fn with_baud_rate(&self, baud_rate: u32) -> PirateMIDIDevice {
        Self {
            vid: self.vid,
            pid: self.pid,
            timeout: self.timeout,
            baud_rate,
            builder: self.builder.clone(),
        }
    }

    /// By default we attempt to connect with devices with the constant BAUD_RATE
    /// However, this method exists for if this changes in the future.
    pub fn with_timeout(&self, timeout: Duration) -> PirateMIDIDevice {
        Self {
            vid: self.vid,
            pid: self.pid,
            timeout,
            baud_rate: self.baud_rate,
            builder: self.builder.clone(),
        }
    }

    pub fn with_serialport_builder(&self, builder: SerialPortBuilder) -> PirateMIDIDevice {
        Self {
            vid: self.vid,
            pid: self.pid,
            timeout: self.timeout,
            baud_rate: self.baud_rate,
            builder: Some(builder),
        }
    }

    /// Send a specific command to a device via the current serial configuration
    pub fn send(&self, command: Command) -> Result<Response, Error> {
        // attempt to find the device
        let serial_device = match &self.builder {
            Some(builder) => Ok(builder.clone()),
            None => self.find_device(),
        };

        match serial_device {
            Ok(device) => match device.open() {
                Ok(mut port) => {
                    // must explicitly set DTR flag for windows
                    match port.write_data_terminal_ready(true) {
                        Ok(_) => (),
                        Err(err) => return Err(crate::Error::SerialError(err.description)),
                    }

                    // if we have a broken pipe error, report it here.
                    match send_commands(&mut port, command.clone(), false) {
                        Err(inner) => Err(inner),
                        Ok(buffer) => {
                            // match our response to our request
                            let result = match command {
                                Command::Check => {
                                    match serde_json::from_str::<check::CheckResponse>(
                                        &trim_response(&buffer),
                                    ) {
                                        Ok(result) => Response::Check(result),
                                        Err(err) => return Err(Error::JsonError(err)),
                                    }
                                }
                                Command::Control(args) => {
                                    let result = match args {
                                        // when entering bootloader, we expect a blank response or "ok"
                                        ControlArgs::EnterBootloader => {
                                            if trim_response(&buffer) == "ok"
                                                || trim_response(&buffer).is_empty()
                                            {
                                                Ok(())
                                            } else {
                                                Err(Error::CommandError(trim_response(&buffer)))
                                            }
                                        }
                                        // otherwise, we expect "ok" as a response
                                        _ => {
                                            if trim_response(&buffer) == "ok" {
                                                Ok(())
                                            } else {
                                                Err(Error::CommandError(trim_response(&buffer)))
                                            }
                                        }
                                    };
                                    Response::Control(result)
                                }
                                Command::DataRequest(subreq) => match subreq {
                                    DataRequestArgs::GlobalSettings => Response::DataRequest(
                                        DataRequestResponse::GlobalSettings(trim_response(&buffer)),
                                    ),
                                    DataRequestArgs::BankSettings(_) => {
                                        match serde_json::from_str::<bank::BankSettings>(
                                            &trim_response(&buffer),
                                        ) {
                                            Ok(result) => Response::DataRequest(
                                                DataRequestResponse::BankSettings(result),
                                            ),
                                            Err(err) => return Err(Error::JsonError(err)),
                                        }
                                    }
                                },
                                Command::DataTransmitRequest(_) => {
                                    Response::DataTransmit(trim_response(&buffer))
                                }
                                Command::Reset => Response::Reset(trim_response(&buffer)),
                            };
                            Ok(result)
                        }
                    }
                }
                Err(err) => Err(Error::SerialError(err.to_string())),
            },
            Err(err) => Err(err),
        }
    }

    fn find_device(&self) -> Result<SerialPortBuilder, Error> {
        match available_ports() {
            Ok(ports) => {
                for p in ports {
                    if let SerialPortType::UsbPort(usb_info) = p.port_type {
                        if usb_info.vid == self.vid && usb_info.pid == self.pid {
                            return Ok(
                                serialport::new(p.port_name, self.baud_rate).timeout(self.timeout)
                            );
                        }
                    }
                }
                Err(Error::SerialError("unable to locate device".to_string()))
            }
            Err(err) => Err(Error::SerialError(err.to_string())),
        }
    }
}

fn trim_response(response: &str) -> String {
    response
        .trim_start()
        .trim_end()
        .trim_start_matches(char::is_numeric)
        .trim_start_matches(',')
        .trim_end_matches('~')
        .to_string()
}

fn send_commands(
    port: &mut Box<dyn SerialPort>,
    command: Command,
    backwards_compatable: bool,
) -> Result<String, crate::Error> {
    // setup output
    let mut buffer = String::new();

    // turn our commands into a series of commands
    for (i, sub_cmd) in command.format(backwards_compatable).iter().enumerate() {
        // clear buffer before we iterate
        if !buffer.is_empty() {
            let _ = &buffer.clear();
        }

        // transmit command
        match backwards_compatable {
            true => {
                // Support BridgeOS 1.0
                trace!("tx: {i},{sub_cmd}~");
                match port.write(format!("{i},{sub_cmd}~").as_bytes()) {
                    Ok(_) => (),
                    Err(ref e) if e.kind() == ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => {
                // Support BridgeOS 2.0
                trace!("tx: {sub_cmd}~");
                match port.write(format!("{sub_cmd}~").as_bytes()) {
                    Ok(_) => (),
                    Err(ref e) if e.kind() == ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }

        match port.read_to_string(&mut buffer) {
            Ok(_) => (),
            Err(e) if e.kind() == ErrorKind::TimedOut => (),
            Err(e) if e.kind() == ErrorKind::BrokenPipe => {
                match &command {
                    Command::Control(sub) => match sub {
                        // these commands will break the pipe on purpose so don't log it as an error
                        ControlArgs::DeviceRestart
                        | ControlArgs::EnterBootloader
                        | ControlArgs::FactoryReset => (),
                        _ => return Err(Error::BrokenPipeError(e)),
                    },
                    _ => return Err(Error::BrokenPipeError(e)),
                };
            }
            Err(e) => return Err(Error::ReadError(e)),
        };

        // attempt backwards compatability if it's not enabled already
        if buffer == "no id number\n~\0" && !backwards_compatable {
            return send_commands(port, command, true);
        }
    }

    // return the buffer
    trace!("rx: {}", buffer);
    Ok(buffer)
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("unable to read from serial pipe")]
    ReadError(#[from] std::io::Error),
    #[error("unable to parse json from device")]
    JsonError(serde_json::Error),
    #[error("there was an issue with the serial connection")]
    SerialError(String),
    #[error("the device returned an unexpected response")]
    CommandError(String),
    #[error("device became disconnected")]
    BrokenPipeError(std::io::Error),
}
