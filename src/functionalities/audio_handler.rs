


// use std::sync::{Mutex, Arc};

// use std::{sync::{Arc, Mutex}};

use cpal::{traits::{DeviceTrait, StreamTrait}};

use super::device_handler::DeviceManager;

pub struct AudioHandler{
    device_manager: DeviceManager,
    channel: cpal::ChannelCount,
    samplerate: cpal::SampleRate,
    buffersize: cpal::BufferSize,
    // buffer: Arc<Mutex<Vec<i16>>>,
}
impl AudioHandler{
    pub fn new(device_manager:DeviceManager) -> AudioHandler {
        let channel:cpal::ChannelCount = 0;
        let samplerate:cpal::SampleRate = cpal::SampleRate(0);
        let buffersize:cpal::BufferSize = cpal::BufferSize::Fixed(128);
        // let buffer = Arc::new(Mutex::new(Vec::new()));

        let mut audio_handler = AudioHandler{
            device_manager,
            channel,
            samplerate,
            buffersize,
            // buffer,
        };
        audio_handler.setup();
        audio_handler
    }
    pub fn record<F>(&self, callback: F) -> () 
    where
        F: Fn(&[i16]) + Send + 'static,
    {
        // let localbuf = self.buffer.clone();
        // println!("buffer size: {:?}", buf);
        
        let device = self.device_manager.get_device();
        let device = &*device.lock().unwrap();

        let stream_config = cpal::StreamConfig{
            channels: self.channel,
            sample_rate: self.samplerate,
            buffer_size: self.buffersize,
        };

        let stream = device.build_input_stream(
            &stream_config,
            move |data: &[i16], _| {
                // let buf = &mut *localbuf.lock().unwrap();
                let data_vec = Vec::from(data);
                callback(&data_vec);

                println!("data: {:?}", data_vec.len());
                // println!("data: {:?}", data_vec);
                // buf.copy_from_slice(data);
                // let arr = data.try_into().unwrap();
            },
            move |err| {
                eprintln!("an error occurred on input stream: {}", err);
            },
            None,
        ).unwrap();
        
        println!("recording...");
        stream.play().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));
        stream.pause().unwrap();

        // let localbuf = self.buffer.clone();
        // let buf = &*localbuf.lock().unwrap();
        // println!("data: {:?}", buf);

    }

    // pub fn get_buffer(&self) -> Arc<Mutex<Vec<i16>>> {
    //     self.buffer.clone()
    // }

    // pub fn new(device_manager: Arc<Mutex<*const DeviceManager>>) -> AudioHandler {
    // // pub fn new(device_manager: *const DeviceManager) -> AudioHandler {
        
    //     let device_mgr_ptr_clone = *device_manager.lock().unwrap();

    //     let mut audio_handler = AudioHandler{
    //         device_manager: device_mgr_ptr_clone,
    //         channel_out:0,
    //         channel_in:0,
    //     };
    //     audio_handler.setup();
    //     audio_handler
    // }

    // pub fn record(&self) -> () {
    //     // let mut buf = [0.0; 256];
    //     // println!("buffer size: {:?}", self.input_config);

    //     // let mut buf = vec![0.0; self.input_config.buffer_size() as usize];

    //     // let mut stream = self.device.build_input_stream(
    //     //     &self.format,
    //     //     move |data, _| {
    //     //         buf = data.to_vec();
    //     //     },
    //     //     move |err| {
    //     //         eprintln!("an error occurred on input stream: {}", err);
    //     //     },
    //     // ).unwrap();
    //     // stream.play().unwrap();
    // }


    // pub fn copy_from_slice(&self, data: &[i16]) -> () {
    //     let buf = &mut *self.buffer.lock().unwrap();
    //     buf.copy_from_slice(data);
    // }



    fn setup(&mut self) -> (){
        self.channel = self.device_manager.get_channel();
        self.samplerate = self.device_manager.get_samplerate();
    }


}


