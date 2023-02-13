
use cpal::{traits::{DeviceTrait, HostTrait}};
use std::io;

pub struct DeviceManager{
    pub host: cpal::Host,
    // pub device: cpal::Device,
    pub device_type: String,
    pub device_list: Vec<cpal::Device>,
    pub device_id: usize,
}
impl DeviceManager {
    pub fn new_output(id: usize) -> DeviceManager {
        let host = cpal::default_host();
        // let device = host.default_output_device().expect("no output device available");
        let device_type = String::from("output");
        let device_list:Vec<cpal::Device> = Vec::new();
        let device_id: usize = id;
        
        let mut DeviceMngr = DeviceManager{
            host: host,
            // device: device,
            device_type: device_type,
            device_list: device_list,
            device_id: device_id,
        };
        DeviceMngr.list_devices();
        DeviceMngr
    }

    pub fn new_input(id: usize) -> DeviceManager {
        let host = cpal::default_host();
        // let device = host.default_input_device().expect("no input device available");
        let device_type = String::from("input");
        let device_list:Vec<cpal::Device> = Vec::new();
        let device_id: usize = id;
        let mut DeviceMngr = DeviceManager{
            host: host,
            // device: device,
            device_type: device_type,
            device_list: device_list,
            device_id: device_id,
        };
        DeviceMngr.list_devices();
        DeviceMngr
    }
    
    pub fn list_devices(&mut self) -> () {
        // let mut temp_devices:Vec<cpal::Device>;
        let default_device: cpal::Device;

        if &self.device_type == "output" {
            default_device = self.host.default_output_device().expect("no output device available");
        }else{
            default_device = self.host.default_input_device().expect("no input device available");
        }
        let default_device_name = default_device.name().unwrap();
        
        self.device_list.push(default_device);
        // temp_devices.push(default_device);
        for device in self.host.devices().unwrap() {
            if device.name().unwrap() != default_device_name {
                self.device_list.push(device);
            }
        }
        if self.device_list.len() <= self.device_id {
            println!("No devices found at id: {:?}, returning to default", self.device_id);
            self.device_id = 0;
        }
    }

    pub fn change_device(&mut self) -> () {
        let mut i = 0;

        if self.device_list.len() == 0 {
            self.list_devices();
        }

        println!("Device list: ");
        print!("*");
        for device in &self.device_list {
            let channel_input = self.get_channel_in(device);
            let channel_output = self.get_channel_out(device);
            
            print!("{:?}. ", i);
            print!("[in: {:?}] ", channel_input);
            print!("[out: {:?}] ", channel_output);
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

    fn get_channel_in(&self, device:&cpal::Device) -> u16 {
        let channel_input = device.default_input_config();
        match channel_input {
            Ok(channel_input) => {
                return channel_input.channels().clone();
            },
            Err(_) => {
                return 0;
            }
        }
    }
    
    fn get_channel_out(&self, device:&cpal::Device) -> u16 {
        let channel_output = device.default_output_config();
        match channel_output {
            Ok(channel_output) => {
                return channel_output.channels().clone();
            },
            Err(_) => {
                return 0;
            }
        }
    }
    
    
    pub fn get_device(&self) -> &cpal::Device {
        return &self.device_list[self.device_id];
    }



}