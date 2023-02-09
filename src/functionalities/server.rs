
pub struct ServerHandler{
    port: String,
    deviceManager: DeviceManager,
}
pub impl Runner for ServerHandler {
    fn new(&self, port:String) {
        println!("server is running");
    }
}