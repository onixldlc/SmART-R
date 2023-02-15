use cpal::traits::DeviceTrait;
use std::net::UdpSocket;
use std::{sync::{Arc, Mutex}};
use std::thread;
use std::io;
use std::time::Duration;
use bincode::{deserialize};


use crate::functionalities::device_handler::DeviceManager;
use crate::functionalities::parser::SmartHandler;

pub struct ServerHandler{
    address: String,
    port: String,
    device_manager: DeviceManager,
    keep_running: Arc<Mutex<bool>>,
    socket: UdpSocket,
}


impl ServerHandler {
    pub fn new(args: &SmartHandler)  -> Self {
        let device_id:usize = args.device_id.clone().unwrap().parse().unwrap();
        let port = args.port.clone().unwrap();
        let address = "0.0.0.0:".to_string() + &port;
        let mut device_manager = DeviceManager::new_output(device_id);

        if args.device_select {
            device_manager.change_device();
        }

        let server_handler = ServerHandler{
            address: address.clone(),
            port: port,
            device_manager: device_manager,
            keep_running: Arc::new(Mutex::new(true)),
            socket: UdpSocket::bind(address).unwrap(),
        };

        let device = server_handler.device_manager.get_device();
        let device = device.lock().unwrap();

        println!("server configs:");
        println!("\t port: {:?}", server_handler.port);
        println!("\t deviceID: {:?}", server_handler.device_manager.device_id);
        println!("\t deviceName: {:?}", device.name().unwrap());

        return server_handler;
    }

    pub fn run(self) -> (){
        // let port = self.port.clone();
        let address = self.address.clone();

        println!("binding to address: {} ...", address);

        let keep_running = self.keep_running.clone();
        let keep_running_clone = Arc::clone(&keep_running);
        // let keep_running_clone2 = Arc::clone(&keep_running);

        // let is_busy = Arc::new(Mutex::new(false));
        // let is_busy_clone = Arc::clone(&is_busy);
        // let is_busy_clone2 = Arc::clone(&is_busy);


        let socket = self.socket.try_clone().unwrap();

        
        let handle = thread::spawn(move || {
        // thread::spawn(move || {
            let socket = socket;
            socket.set_read_timeout(Some(Duration::from_secs(5))).unwrap();

            let timing = socket.read_timeout().unwrap();
            println!("{:?}", timing);

            // socket.set_read_timeout(Some(Duration::from_millis(500))).unwrap();
            let mut buf = [0; 1024];

            // loop {
            //     let keep_running = *keep_running_clone.lock().unwrap();
            //     match initial_socket.recv_from(&mut buf){
            //         Ok((_, src)) => println!("ok"),
            //         err => println!("no connection yet"),
            //     }
            // }

            let mut is_busy = false;
            loop {
                let keep_running = *keep_running_clone.lock().unwrap();
                println!("is_busy: {:?}, keep_running: {:?}", is_busy, keep_running);
                match socket.recv_from(&mut buf) {
                    Ok((_, src)) => {
                        if !is_busy {
                            socket.set_read_timeout(Some(Duration::from_millis(500))).unwrap();
                            is_busy = true;
                        }
                        let data: Vec<i16> = deserialize(&buf).unwrap();
                        println!("received data from {}: {:?}", src, &data[..]);
                    }
                    Err(_e) => {
                        if keep_running {
                            println!("no data yet!");
                            if is_busy {
                                socket.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
                                is_busy = false;
                            }
                            // println!("could not receive a datagram: {} {:?}", e, keep_running);
                        }
                        
                        else {
                            println!("server thread stopped...");
                            break;
                        }
                    }
                }
            }

            // loop {
            //     let keep_running = *keep_running_clone.lock().unwrap();
            //     match socket.recv_from(&mut buf) {
            //         Ok((_, src)) => {
            //             let data: Vec<i16> = deserialize(&buf).unwrap();
            //             println!("received data from {}: {:?}", src, &data[..]);
            //         }
            //         Err(_e) => {
            //             if keep_running {
            //                 println!("no data yet!");
            //                 // println!("could not receive a datagram: {} {:?}", e, keep_running);
            //             }
            //             else {
            //                 println!("server thread stopped...");
            //                 break;
            //             }
            //         }
            //     }
            // }
        });

        println!("press enter to stop server!");
        io::stdin().read_line(&mut String::new()).unwrap();
        self.stop(handle, keep_running);
        
        // match handle.join() {
        //     Ok(_) => println!("server thread joined!"),
        //     Err(_) => println!("server thread panicked!"),
        // }
        // self.stop(handle);
    }

    pub fn stop(&self, thread: thread::JoinHandle<()>, keep_running:Arc<Mutex<bool>>) -> (){

        println!("stoping server...");
        thread::spawn(move || {
            let mut keep_running = keep_running.lock().unwrap();
            *keep_running = false;
        }).join().unwrap();
        thread.join().unwrap();
        println!("server stopped...");
    }

    // fn initial_encounter(&self){
    //     let keep_running_clone = Arc::clone(&self.keep_running);
    //     let initial_socket = self.socket.try_clone().unwrap();
    //     let mut buf = [0; 1024];

    //     loop {
    //         let keep_running = *keep_running_clone.lock().unwrap();
    //         match initial_socket.recv_from(&mut buf){
    //             Ok((_, src)) => println!("ok"),
    //             err => println!("no connection yet"),
    //         }
    //     }
    // }

    // fn data_transfer(&self){

    // }


}