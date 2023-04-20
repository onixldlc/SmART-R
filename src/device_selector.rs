use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::Device;
use std::io;

pub enum DeviceType {
    Input,
    Output,
}

pub fn select_device() -> Result<Device> {
    let host = cpal::default_host();
    let mut devices: Vec<_> = host.devices()?.collect();

    for (i, device) in devices.iter().enumerate() {
        println!("{}: {}", i, device.name()?);
    }

    // read an integer from console
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let index = input.trim().parse::<usize>()?;
    Ok(devices.remove(index))
}

pub fn select_device_by_id(id: Option<usize>, device_type: DeviceType) -> Result<Device> {
    let host = cpal::default_host();

    match id {
        None => match device_type {
            DeviceType::Input => Ok(host
                .default_input_device()
                .with_context(|| "No default input device found")?),
            DeviceType::Output => Ok(host
                .default_output_device()
                .with_context(|| "No default output device found")?),
        },
        Some(id) => {
            let mut devices: Vec<_> = host.devices()?.collect();
            Ok(devices.remove(id))
        }
    }
}
