use cpal::traits::DeviceTrait;
use std::net::UdpSocket;
use std::{sync::{Arc, Mutex}};
use std::thread;
use std::io;
use bincode::{serialize};

use crate::functionalities::device_handler::DeviceManager;
use crate::functionalities::parser::SmartHandler;
use crate::functionalities::audio_handler::AudioHandler;

pub struct ClientHandler{
    keep_running: bool,
    address: String,
    port: String,
    device_manager: DeviceManager,
}
impl ClientHandler {
    pub fn new(args: &SmartHandler) -> Self {
        if args.address == None {
            panic!("Error address is not set");
        }

        let device_id:usize = args.device_id.clone().unwrap().parse().unwrap();
        let port = args.port.clone().unwrap();
        let address = args.address.clone().unwrap();
        let mut device_manager = DeviceManager::new_input(device_id);
        if args.select_device {
            device_manager.change_device();
        }

        let client_handler = ClientHandler{
            keep_running: true,
            port: port,
            address: address,
            device_manager: device_manager,
        };

        let device = client_handler.device_manager.get_device();
        let device = device.lock().unwrap();

        println!("client configs:");
        println!("\t address: {:?}", client_handler.address);
        println!("\t port: {:?}", client_handler.port);
        println!("\t deviceID: {:?}", client_handler.device_manager.device_id);
        println!("\t deviceName: {:?}", device.name().unwrap());

        return client_handler;
    }

    pub fn run(mut self) -> (){
        self.keep_running = true;
        let port = self.port.clone();
        let address = self.address.clone() + ":" + &port;
        let address2 = self.address.clone() + ":" + &port;
        
        println!("sending to address: {}...", address);

        let keep_running = Arc::new(Mutex::new(true));
        // let keep_running_clone = Arc::clone(&keep_running);

        let device_mgr = self.device_manager.clone();

        let socket = UdpSocket::bind("127.0.0.1:5545").expect("couldn't bind to address");

        let handle = thread::spawn(move || {
            let audio_handler = AudioHandler::new(device_mgr);
            // socket.connect(address.clone()).expect("connect function failed");
            // let audio_socket = &socket.try_clone().expect("couldn't clone socket");

            audio_handler.record(move|data: &[i16]| {
                let serialized_data = serialize(data).unwrap();
                match socket.send_to(&serialized_data, &address2.clone()) {
                    Ok(_) => {
                        println!("sent data: {:?} to {:?}", &data[..], &address2.clone())
                    },
                    Err(_e) => println!("something went wrong when sending data"),
                }
            });
            // let audio_buf = audio_handler.get_buffer();
            // let audio_buf = &*audio_buf.lock().unwrap();
            // loop {
            //     let keep_running = *keep_running_clone.lock().unwrap();
            //     let buf = &b"hello server"[..];
            //     if keep_running {
            //         match audio_socket.send_to(buf, &address.clone()) {
            //             Ok(_) => {
            //                 // println!("{:?}", &audio_buf)
            //                 println!("sent data: {:?} to {:?}", &buf[..], address)
            //             },
            //             Err(_e) => println!("something went wrong when sending data"),
            //         }
            //     }else{
            //         break;
            //     }
            //     thread::sleep(time::Duration::from_secs(3));
            // }
        });

        println!("press enter to stop client!");
        io::stdin().read_line(&mut String::new()).unwrap();
        self.stop(handle, keep_running);
    }

    pub fn stop(&self, thread: thread::JoinHandle<()>, keep_running:Arc<Mutex<bool>>) -> (){
        println!("stoping client...");
        thread::spawn(move || {
            let mut keep_running = keep_running.lock().unwrap();
            *keep_running = false;
        }).join().unwrap();
        thread.join().unwrap();
        println!("client stopped...");
    }
}