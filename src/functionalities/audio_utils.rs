// use rodio::source::Source;
// use rodio::Sink;

// pub struct Recorder{
//     source: Source,
//     sink: Sink,
// }
// pub impl Recorder {
//     fn new(&self) {
//         self.sink = Sink::new();
//         self.source = Source::new();
//         self.sink.append(self.source);
//         println!("recorder is running");
//     }
//     fn start(&self) {
        
//         println!("Press Enter to stop recording and playback.");
//     }
// }


// pub struct DeviceManager {
//     devices: rodio::Device,
// }
// pub impl DeviceManager {
//     fn new(&self) {
//         self.devices = rodio::Device::default_input_format();
//     }
//     fn get_devices(&self) -> rodio::Device {
//         return self.devices;
//     }
// }