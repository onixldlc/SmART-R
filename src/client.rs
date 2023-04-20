use std::net::{SocketAddr, UdpSocket};

use anyhow::Result;
use cpal::traits::DeviceTrait;
use cpal::{Data, Device, Stream, StreamConfig};
use log::{debug, error, info};

use crate::cli::HandlerArgs;
use crate::device_selector::{select_device, select_device_by_id, DeviceType};

pub struct ClientHandler {
    address: SocketAddr,
    device: Device,
}

impl ClientHandler {
    pub fn new(args: HandlerArgs) -> Result<Self> {
        let address = SocketAddr::from((args.address, args.port));

        let device = if args.select_device {
            select_device()
        } else {
            select_device_by_id(args.device_id, DeviceType::Input)
        }?;

        let handler = ClientHandler { address, device };

        info!("client configs:");
        info!("\t address: {:?}", &handler.address);
        info!("\t deviceName: {:?}", &handler.device.name()?);

        Ok(handler)
    }

    pub fn create_stream(&self) -> Result<Stream> {
        let config = self.device.default_input_config()?;
        let socket = UdpSocket::bind("127.0.0.1:0")?;

        let address = self.address;
        let stream = self.device.build_input_stream_raw(
            &StreamConfig {
                channels: config.channels(),
                sample_rate: config.sample_rate(),
                buffer_size: cpal::BufferSize::Default,
            },
            config.sample_format(),
            move |data: &Data, _: &cpal::InputCallbackInfo| match socket
                .send_to(data.bytes(), address)
            {
                Ok(size) => debug!(
                    "got {:?} and sent {:?} bytes to {:?}",
                    data.bytes().len(),
                    size,
                    &address
                ),
                Err(e) => error!("something went wrong when sending data: {}", e),
            },
            |err| error!("something went wrong: {}", err),
            None,
        )?;

        Ok(stream)
    }
}
