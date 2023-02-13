use cpal::traits::DeviceTrait;

use crate::functionalities::device_handler::DeviceManager;
use crate::functionalities::parser::SmartHandler;

pub struct ServerHandler{
    port: String,
    device_manager: DeviceManager,
}


impl ServerHandler {
    pub fn new(args: &SmartHandler) -> () {
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

        println!("server is running");
        println!("params:");
        println!("\t port: {:?}", server_handler.port);
        println!("\t deviceID: {:?}", server_handler.device_manager.device_id);
        println!("\t deviceName: {:?}", device.name().unwrap());
    }

    // pub fn run(mut self) -> (){
    //     self.keepRunning = true;
    //     println!("running Server mode\n");
    // }

    // pub fn stop(mut self) -> (){
    //     self.keepRunning = false;
    //     println!("stopping Server mode\n");
    // }


}