use std::process::Command;
use crate::{Result, SwitcherError};

pub enum InputSource {
    Vga1,
    DisplayPort1,
    Hdmi1,
    Hdmi2
}

impl InputSource {
    pub fn value(&self) -> u8 {
        match *self{
            InputSource::Vga1 => 0x01,
            InputSource::DisplayPort1 => 0x0f,
            InputSource::Hdmi1 => 0x11,
            InputSource::Hdmi2 => 0x12,
        }
    }
    pub fn from_value(value: u8) -> Option<InputSource>{
        match value {
            0x01 => Some(InputSource::Vga1),
            0x0f => Some(InputSource::DisplayPort1),
            0x11 => Some(InputSource::Hdmi1),
            0x12 => Some(InputSource::Hdmi2),
            _ => Option::None
        }
    }
}

pub fn get_input_source() -> Result<InputSource>{
    const FEATURE:u8 = 0x60;

    let result = Command::new("ddcutil")
        .args(&["getvcp",
            "--model",
            "24G1WG4",
            "--terse",
            &format!("0x{:x}",FEATURE)])
        .output()?;

    if !result.status.success() {
        return Err(SwitcherError::ExitCode(result.status.code().unwrap_or(255)));
    }

    let response = String::from_utf8_lossy(&result.stdout).into_owned();

    let value_str: &str = response.split_ascii_whitespace()
        .nth(3)
        .ok_or_else(|| SwitcherError::DDCError(String::from("Invalid response")))?;

    let value = u8::from_str_radix(value_str.trim_start_matches("x"),16)
        .map_err(|_| SwitcherError::DDCError(String::from("Can't parse response")))?;

    let input = InputSource::from_value(value)
        .ok_or_else(|| SwitcherError::DDCError(format!("Invalid InputSource type {}",&value)))?;

    Ok(input)
}

pub fn set_input_source(source: InputSource) -> Result<bool>{
    const FEATURE:u8 = 0x60;

    let status = Command::new("ddcutil")
        .args(&["setvcp",
            "--model",
            "24G1WG4",
            "--terse",
            &format!("0x{:x}",FEATURE),
            &format!("0x{:x}",source.value())])
        .status()?;

    Ok(status.success())
}