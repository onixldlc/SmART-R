use cpal::traits::DeviceTrait;
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::io;
use std::time::Duration;


use crate::functionalities::device_handler::DeviceManager;
use crate::functionalities::parser::SmartHandler;

pub struct ServerHandler{
    port: String,
    device_manager: DeviceManager,
}


impl ServerHandler {
    pub fn new(args: &SmartHandler)  -> Self {
        let device_id:usize = args.device_id.clone().unwrap().parse().unwrap();
        let port = args.port.clone().unwrap();
        let mut device_manager = DeviceManager::new_output(device_id);

        if args.device_select {
            device_manager.change_device();
        }

        let server_handler = ServerHandler{
            port: port,
            device_manager: device_manager,
        };

        let device = server_handler.device_manager.get_device().clone();

        println!("server configs:");
        println!("\t port: {:?}", server_handler.port);
        println!("\t deviceID: {:?}", server_handler.device_manager.device_id);
        println!("\t deviceName: {:?}", device.name().unwrap());

        return server_handler;
    }

    pub fn run(self) -> (){
        let port = self.port.clone();
        let address = "0.0.0.0:".to_string() + &port;

        println!("binding to address: {} ...", address);

        let keep_running = Arc::new(Mutex::new(true));
        let keep_running_clone = Arc::clone(&keep_running);
        
        let handle = thread::spawn(move || {
        // thread::spawn(move || {
            let socket = UdpSocket::bind(address).unwrap();
            socket.set_read_timeout(Some(Duration::from_millis(500))).unwrap();
            let mut buf = [0; 256];
            loop {
                let keep_running = *keep_running_clone.lock().unwrap();
                match socket.recv_from(&mut buf) {
                    Ok((_, src)) => {
                        println!("received data from {}: {:?}", src, &buf[..]);
                    }
                    Err(_e) => {
                        if keep_running {
                            println!("no data yet!");
                            // println!("could not receive a datagram: {} {:?}", e, keep_running);
                        }
                        else {
                            println!("server thread stopped...");
                            break;
                        }
                    }
                }
            }
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


}