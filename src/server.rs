use crate::device_selector::{select_device, select_device_by_id, DeviceType};
use core::slice;

use cpal::traits::DeviceTrait;
use cpal::{Data, Device, Stream, StreamConfig};

use std::net::{SocketAddr, UdpSocket};

use crate::cli::HandlerArgs;
use anyhow::{Context, Result};
use log::{debug, error, info};

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
                    let mut buf = vec![0_u8; data.len() * 4 / 2];
                    match &socket.recv_from(&mut buf) {
                        Ok((size, addr)) => {
                            debug!("got {} bytes from {}", size, addr,)
                        }
                        Err(e) => {
                            error!("recv function failed: {}", e);
                        }
                    }
                    // duplicate the buffer to make it stereo
                    let f32_vec: &[f32] =
                        unsafe { slice::from_raw_parts(buf.as_ptr() as *const f32, buf.len() / 4) };
                    let mut duped = f32_vec
                        .iter()
                        .flat_map(|x| vec![*x, *x])
                        .collect::<Vec<f32>>();
                    // prevent buf being dropped as it's backing f32_vec (hopefully)
                    let _ = &buf;
                    duped.swap_with_slice(data.as_slice_mut().unwrap())
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
