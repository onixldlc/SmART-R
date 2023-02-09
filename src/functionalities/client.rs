
pub struct ClientHandler{
    address: String,
    port: String,
    deviceManager: DeviceManager,
}
pub impl Runner for ClientHandler {
    fn new(&self, port:String, address:String) {
        if address == "none" {
            println!("address is not set");
            throw
        }
        println!("client is running");
    }
}