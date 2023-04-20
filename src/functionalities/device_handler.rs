
use cpal::{traits::{DeviceTrait, HostTrait}};
use std::{sync::{Mutex, Arc, MutexGuard}, io};

#[derive(Clone)]
pub struct DeviceManager{
    pub host: Arc<Mutex<cpal::Host>>,
    pub device_type: String,
    pub device_list: Vec<Arc<Mutex<cpal::Device>>>,
    pub device_id: usize,
}
impl DeviceManager {
    pub fn new_output(id: usize) -> DeviceManager {
        let host = Arc::new(Mutex::new(cpal::default_host()));
        let device_type = String::from("output");
        let device_list:Vec<Arc<Mutex<cpal::Device>>> = Vec::new();
        let device_id: usize = id;
        
        let mut DeviceMngr = DeviceManager{
            host: host,
            device_type: device_type,
            device_list: device_list,
            device_id: device_id,
        };
        DeviceMngr.list_devices();
        DeviceMngr
    }

    pub fn new_input(id: usize) -> DeviceManager {
        let host = Arc::new(Mutex::new(cpal::default_host()));
        let device_type = String::from("input");
        let device_list:Vec<Arc<Mutex<cpal::Device>>> = Vec::new();
        let device_id: usize = id;
        let mut DeviceMngr = DeviceManager{
            host: host,
            device_type: device_type,
            device_list: device_list,
            device_id: device_id,
        };
        DeviceMngr.list_devices();
        DeviceMngr
    }





    pub fn list_devices(&mut self) -> () {
        let mut temp_devices:Vec<Arc<Mutex<cpal::Device>>> = Vec::new();
        let default_device: Arc<Mutex<cpal::Device>>;
        let local_host = &*self.host.lock().unwrap();

        if &self.device_type == "output" {
            default_device = Arc::new(Mutex::new(local_host.default_output_device().expect("no output device available")));
        }else{
            default_device = Arc::new(Mutex::new(local_host.default_input_device().expect("no input device available")));
        }
        let default_device_name = default_device.lock().unwrap().name().unwrap();

        temp_devices.push(default_device);
        for device in local_host.devices().unwrap() {
            if device.name().unwrap() != default_device_name {
                temp_devices.push(Arc::new(Mutex::new(device)));
            }
        }
        if temp_devices.len() <= self.device_id {
            println!("No devices found at id: {:?}, returning to default", self.device_id);
            self.device_id = 0;
        }
        self.device_list = temp_devices;
    }
    
    pub fn change_device(&mut self) -> () {
        let mut i = 0;

        if self.device_list.len() == 0 {
            self.list_devices();
        }  

        // println!("Device list: {:?}", self.device_list.len());

        println!("Device list: ");
        print!("*");
        for device in &self.device_list {
            let device = &*device.lock().unwrap();
            let channel_input = self.get_channel_in(Some(device));
            let channel_output = self.get_channel_out(Some(device));
            // let _buffer_input = self.get_buffer_in(Some(device));
            // let _buffer_output = self.get_buffer_out(Some(device));


            print!("{:?}. ", i);
            // print!("[bf in: {:?}] ", buffer_input);
            // print!("[bf out: {:?}] ", buffer_output);
            print!("[ch in: {:?}] ", channel_input);
            print!("[ch out: {:?}] ", channel_output);
            println!("{:?}", device.name().unwrap());

            i += 1;
        }
        self.device_id = self.select_device();
    }

    fn select_device(&self) -> usize {
        let mut input = String::new();
        let device_id: u32;
        println!("Select device id: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "" {
            return 0;
        }
        device_id = input.trim().parse().expect("Please type a number!");
        if self.device_list.len() <= device_id as usize {
            println!("No devices found at id: {:?}, returning to default", self.device_id);
            return 0;
        }
        return device_id as usize
    }

    pub fn get_channel(&self) -> cpal::ChannelCount{
        if self.device_type == "output" {
            return self.get_channel_out(None);
        }else{
            return self.get_channel_in(None);
        }
    }

    pub fn get_channel_in(&self, device: Option<&cpal::Device>) -> u16 {
        // let device = device.map_or_else(|| {
        //     let curr_device = self.get_device();
        //     let device = curr_device.lock().unwrap();
        //     &*device
        // }, |d| d);
        

        // let mut binding: Arc<Mutex<cpal::Device>>;
        // let mut curr_device:MutexGuard<cpal::Device>;
        // let device = device.unwrap_or_else(move || {
        //     binding = self.get_device();
        //     curr_device = binding.lock().unwrap();
        //     &*curr_device
        // });
        
        // let device = device.unwrap_or_else(|| -> &cpal::Device {
        //     let device = self.get_device();
        //     let device = &*device.lock().unwrap();
        //     return device
        // });

        let selected_device: &cpal::Device;
        let binding: Arc<Mutex<cpal::Device>>;
        let curr_device: MutexGuard<cpal::Device>;

        if device.is_none() {
            binding = self.get_device();
            curr_device = binding.lock().unwrap();
            selected_device = &*curr_device;

        }else{
            selected_device = device.unwrap();
        }

        // let local_device: &cpal::Device;
        
        // local_device = device.unwrap_or_else(|| {
        //     let device = self.get_device();
        //     local_device = &*device.lock().unwrap();
        // });


        let channel_input = selected_device.default_input_config();
        match channel_input {
            Ok(channel_input) => {
                return channel_input.channels().clone();
            },
            Err(_) => {
                return 0;
            }
        }
    }
    
    pub fn get_channel_out(&self, device: Option<&cpal::Device>) -> u16 {
        let selected_device: &cpal::Device;
        let binding: Arc<Mutex<cpal::Device>>;
        let curr_device: MutexGuard<cpal::Device>;

        if device.is_none() {
            binding = self.get_device();
            curr_device = binding.lock().unwrap();
            selected_device = &*curr_device;

        }else{
            selected_device = device.unwrap();
        }
        // let curr_device = self.get_device();
        // let curr_device = curr_device.lock().unwrap();
        // let device = device.unwrap_or_else(|| &*curr_device);

        // let channel_output = device.default_output_config();
        let channel_output = selected_device.default_output_config();
        match channel_output {
            Ok(channel_output) => {
                return channel_output.channels().clone();
            },
            Err(_) => {
                return 0;
            }
        }
    }

    pub fn get_samplerate(&self) -> cpal::SampleRate{
        if self.device_type == "output" {
            return cpal::SampleRate(self.get_samplerate_out());
        }else{
            return cpal::SampleRate(self.get_samplerate_in());
        }
    }

    pub fn get_samplerate_in(&self) -> u32 {
        let device = self.get_device();
        let device = &*device.lock().unwrap();
        let sample_rate = device.default_input_config();
        match sample_rate {
            Ok(sample_rate) => {
                return sample_rate.sample_rate().clone().0;
            },
            Err(_) => {
                return 0;
            }
        }
    }

    pub fn get_samplerate_out(&self) -> u32 {
        let device = self.get_device();
        let device = &*device.lock().unwrap();
        let sample_rate = device.default_output_config();
        match sample_rate {
            Ok(sample_rate) => {
                return sample_rate.sample_rate().clone().0;
            },
            Err(_) => {
                return 0;
            }
        }
    }

    // pub fn get_buffer_in(&self, device: Option<&cpal::Device>) -> cpal::SupportedBufferSize  {
    //     let selected_device: &cpal::Device;
    //     let binding: Arc<Mutex<cpal::Device>>;
    //     let curr_device: MutexGuard<cpal::Device>;

    //     if device.is_none() {
    //         binding = self.get_device();
    //         curr_device = binding.lock().unwrap();
    //         selected_device = &*curr_device;

    //     }else{
    //         selected_device = device.unwrap();
    //     }

    //     let config = selected_device.default_input_config();
    //     match config {
    //         Ok(config) => {
    //             let buffer_size = config.buffer_size().clone();
    //             return buffer_size;
    //         },
    //         Err(_) => {
    //             let temp_channel = 1;
    //             let temp_sample_rate = cpal::SampleRate(44100);
    //             let temp_supported_buffer = cpal::SupportedBufferSize::Range { min: 0, max: 0 };
    //             let temp_sample_format = cpal::SampleFormat::F32;

    //             let config = cpal::SupportedStreamConfig::new(
    //                 temp_channel,
    //                 temp_sample_rate,
    //                 temp_supported_buffer,
    //                 temp_sample_format,
    //             );
    //             let buffer_size = config.buffer_size().clone();
    //             return buffer_size;
    //         }
    //     }


        
    //     // match config {
    //     //     Ok(config) => {
    //     //         let buffer_size = config.buffer_size().clone();
    //     //         buffer_size.
    //     //     },
    //     //     Err(_) => {
    //     //         return 0;
    //     //     }
    //     // }
    // }
    // pub fn get_buffer_out(&self, device: Option<&cpal::Device>) -> cpal::SupportedBufferSize  {
    //     let selected_device: &cpal::Device;
    //     let binding: Arc<Mutex<cpal::Device>>;
    //     let curr_device: MutexGuard<cpal::Device>;

    //     if device.is_none() {
    //         binding = self.get_device();
    //         curr_device = binding.lock().unwrap();
    //         selected_device = &*curr_device;

    //     }else{
    //         selected_device = device.unwrap();
    //     }

    //     let config = selected_device.default_output_config();
    //     match config {
    //         Ok(config) => {
    //             let buffer_size = config.buffer_size().clone();
    //             return buffer_size;
    //         },
    //         Err(_) => {
    //             let temp_channel = 1;
    //             let temp_sample_rate = cpal::SampleRate(44100);
    //             let temp_supported_buffer = cpal::SupportedBufferSize::Range { min: 0, max: 0 };
    //             let temp_sample_format = cpal::SampleFormat::F32;

    //             let config = cpal::SupportedStreamConfig::new(
    //                 temp_channel,
    //                 temp_sample_rate,
    //                 temp_supported_buffer,
    //                 temp_sample_format,
    //             );
    //             let buffer_size = config.buffer_size().clone();
    //             return buffer_size;
    //         }
    //     }
    // }
    
    pub fn get_device(&self) -> Arc<Mutex<cpal::Device>> {
        return self.device_list[self.device_id].clone();
    }


}