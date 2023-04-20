use crate::device_selector::{select_device, select_device_by_id, DeviceType};

use cpal::traits::DeviceTrait;
use cpal::{Data, Device, SampleFormat, Stream, StreamConfig};

use std::net::{SocketAddr, UdpSocket};

use crate::cli::HandlerArgs;
use anyhow::{Context, Result};
use log::{debug, error, info};
use opus::{Channels, Decoder};

pub struct ServerHandler {
    address: SocketAddr,
    device: Device,
    socket: UdpSocket,
}

impl ServerHandler {
    pub fn new(args: HandlerArgs) -> Result<Self> {
        let address = SocketAddr::from((args.address, args.port));

        let device = if args.select_device {
            select_device()
        } else {
            select_device_by_id(args.device_id, DeviceType::Output)
        }?;

        let socket = UdpSocket::bind(address).with_context(|| "Failed to bind socket.")?;
        let handler = ServerHandler {
            address,
            device,
            socket,
        };

        info!("server configs:");
        info!("\t address: {:?}", &handler.address);
        info!("\t deviceName: {:?}", &handler.device.name()?);

        Ok(handler)
    }

    pub fn create_stream(&self) -> Result<Stream> {
        let config = self.device.default_output_config()?;
        let socket = self.socket.try_clone()?;
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
                            error!("recv function failed: {}", e);
                        }
                    }
                },
                |err| {
                    error!("an error occurred on stream: {}", err);
                },
                None,
            )
            .with_context(|| "Failed to create audio stream.")?;
        Ok(stream)
    }
}
