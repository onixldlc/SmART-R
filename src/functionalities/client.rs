use cpal::traits::DeviceTrait;

use crate::functionalities::device_handler::DeviceManager;
use crate::functionalities::parser::SmartHandler;

pub struct ClientHandler{
    address: String,
    port: String,
    device_manager: DeviceManager,
}
impl ClientHandler {
    pub fn new(args: &SmartHandler) {
        if args.address == None {
            panic!("Error address is not set");
        }

        let device_id:usize = args.device_id.clone().unwrap().parse().unwrap();
        let port = args.port.clone().unwrap();
        let address = args.address.clone().unwrap();
        let mut device_manager = DeviceManager::new_input(device_id);
        if args.device_select {
            device_manager.change_device();
        }

        let client_handler = ClientHandler{
            port: port,
            address: address,
            device_manager: device_manager,
        };

        let device = client_handler.device_manager.get_device().clone();

        println!("client is running");
        println!("params:");
        println!("\t address: {:?}", client_handler.address);
        println!("\t port: {:?}", client_handler.port);
        println!("\t deviceID: {:?}", client_handler.device_manager.device_id);
        println!("\t deviceName: {:?}", device.name().unwrap());

    }

    // pub fn run(mut self) -> (){
    //     println!("running Server mode\n");
    // }

    // pub fn stop(mut self) -> (){
    //     println!("stopping Server mode\n");
    // }
}