


// use std::sync::{Mutex, Arc};

// use std::{sync::{Arc, Mutex}};

use std::{thread};

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
        let buffersize:cpal::BufferSize = cpal::BufferSize::Fixed(256);
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
        F: Fn(&[i16]) + Send + Sync + 'static,
    {
        // let localbuf = self.buffer.clone();
        // println!("buffer size: {:?}", buf);
        
        let device = self.device_manager.get_device();

        let channel = self.channel.clone();
        let samplerate = self.samplerate.clone();
        let buffersize = self.buffersize.clone();

        // let stream_config = cpal::StreamConfig{
        //     channels: channel.clone(),
        //     sample_rate: samplerate.clone(),
        //     buffer_size: buffersize.clone(),
        // };

        // let share_callback = Arc::new(Mutex::new(callback));
        // let share_callback_clone = Arc::clone(&share_callback);




        // let _stream = device.build_input_stream(
        //     &stream_config,
        //     move |data: &[i16], _| {
        //         // let buf = &mut *localbuf.lock().unwrap();
        //         let data_vec = Vec::from(data);

        //         let share_callback = &*share_callback.lock().unwrap();
        //         let boxed_callback = Box::new(share_callback) as Box<dyn Fn(&[i16]) + Send>;
        //         boxed_callback(&data_vec);

        //         println!("data: {:?}", data_vec.len());
        //         // println!("data: {:?}", data_vec);
        //         // buf.copy_from_slice(data);
        //         // let arr = data.try_into().unwrap();
        //     },
        //     move |err| {
        //         eprintln!("an error occurred on input stream: {}", err);
        //     },
        //     None,
        // ).unwrap();
        
        println!("recording...");


        thread::spawn(move || {
            let stream_config = cpal::StreamConfig{
                channels: channel.clone(),
                sample_rate: samplerate.clone(),
                buffer_size: buffersize.clone(),
            };

            let device = &*device.lock().unwrap();

            let stream = device.build_input_stream(
                &stream_config, 
                move |data: &[i16], _| {
                    let data_vec = Vec::from(data);

                    callback(&data_vec);
                    // let share_callback_clone = &*share_callback_clone.lock().unwrap();
                    // let boxed_callback2 = Box::new(share_callback_clone) as Box<dyn Fn(&[i16]) + Send>;
                    // boxed_callback2(&data_vec);

                    println!("data: {:?}", data);
                }, 
                move |err| eprint!("something went wrong: {}", err), 
                None,
            ).unwrap();
            stream.play().unwrap();
            loop {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        });
        // stream.play().unwrap();
        // std::thread::sleep(std::time::Duration::from_millis(50));
        // stream.pause().unwrap();

        // let localbuf = self.buffer.clone();
        // let buf = &*localbuf.lock().unwrap();
        // println!("data: {:?}", buf);

    }













    
    pub fn playback<F>(&self, callback: F) -> ()
    where
        F: Fn(&mut cpal::Data) + Send + 'static,

    {
        let device = self.device_manager.get_device();

        let channel = self.channel.clone();
        let samplerate = self.samplerate.clone();
        let buffersize = self.buffersize.clone();
        println!("buffersize: {:?}, samplerate: {:?}, channel: {:?}", buffersize, samplerate, channel);

        thread::spawn(move || {
            let stream_config = cpal::StreamConfig{
                channels: channel.clone(),
                sample_rate: samplerate.clone(),
                buffer_size:cpal::BufferSize::Fixed(128),
            };

            let device = &*device.lock().unwrap();

            let stream = device.build_output_stream_raw(
                &stream_config, 
                cpal::SampleFormat::I16,
                move |data: &mut cpal::Data, _:&cpal::OutputCallbackInfo| {
                    println!("data_len: {:?}", data.len());
                    callback(data);
                    
                    println!("data: {:?}", data);
                }, 
                move |err| eprint!("something went wrong: {}", err), 
                None,
            ).unwrap();
            stream.play().unwrap();
            loop {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        });
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


