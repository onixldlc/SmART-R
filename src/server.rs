use std::net::{SocketAddr, UdpSocket, IpAddr};

use anyhow::{Context, Result};
use cpal::traits::DeviceTrait;
use cpal::{Data, Device, SampleFormat, Stream, StreamConfig};
use log::{debug, error, info};
use opus::{Channels, Decoder};

use crate::cli::HandlerArgs;
use crate::device_selector::{select_device, select_device_by_id, DeviceType};

pub struct ServerHandler {
    address: SocketAddr,
    device: Device,
}

impl ServerHandler {
    pub fn new(args: HandlerArgs) -> Result<Self> {
        let addr = args.address.unwrap_or("0.0.0.0".parse::<IpAddr>().unwrap());
        let address = SocketAddr::from((addr, args.port));

        let device = if args.select_device {
            select_device()
        } else {
            select_device_by_id(args.device_id, DeviceType::Output)
        }?;

        let handler = ServerHandler { address, device };

        info!("Server Configs:");
        info!("\tAddress: {:?}", &handler.address);
        info!("\tDevice Name: {:?}", &handler.device.name()?);

        Ok(handler)
    }

    pub fn create_stream(&self) -> Result<Stream> {
        let socket = UdpSocket::bind(self.address).with_context(|| "Failed to bind socket")?;
        let config = self.device.default_output_config()?;
        let channels = match config.channels() {
            1 => Channels::Mono,
            2 => Channels::Stereo,
            _ => panic!("Unsupported number of channels"),
        };
        let mut decoder = Decoder::new(config.sample_rate().0, channels)?;
        let stream = self
            .device
            .build_output_stream_raw(
                &StreamConfig {
                    channels: config.channels(),
                    sample_rate: config.sample_rate(),
                    buffer_size: cpal::BufferSize::Default,
                },
                config.sample_format(),
                move |data: &mut Data, _: &cpal::OutputCallbackInfo| {
                    let mut buf = vec![0_u8; data.len() * 4];
                    match socket.recv_from(&mut buf) {
                        Ok((size, addr)) => {
                            match data.sample_format() {
                                SampleFormat::F32 => {
                                    decoder
                                        .decode_float(
                                            &buf[..size],
                                            data.as_slice_mut().unwrap(),
                                            false,
                                        )
                                        .unwrap();
                                }
                                SampleFormat::I16 => {
                                    decoder
                                        .decode(&buf[..size], data.as_slice_mut().unwrap(), false)
                                        .unwrap();
                                }
                                _ => {
                                    panic!("Unsupported sample format");
                                }
                            }
                            debug!("got {} bytes from {}", size, addr,)
                        }
                        Err(e) => {
                            error!("Something went wrong when receiving data: {}", e);
                        }
                    }
                },
                |e| {
                    error!("Something went wrong with audio stream: {}", e);
                },
                None,
            )
            .with_context(|| "Failed to create audio stream")?;
        Ok(stream)
    }
}
