#![allow(non_snake_case)]
// use rodio::{Device, Sink};
// use tokio::net::{TcpListener, TcpStream};
// use tokio::prelude::*;
// use std::env;

mod functionalities;
// use functionalities::parser::{parse_args};

fn main() {
    // let args: Vec<_> = env::args().collect();

    // for arg in args.clone() {
    //     println!("{} ,", arg);
    // }
    // println!("\n\n");

    let _ = functionalities::parser::parse_args();
    





    /*
    // Bind the server to the localhost:8000
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    // Get the default input device
    let input_device = rodio::default_input_device().unwrap();

    // Start listening for incoming connections
    let server = listener
        .incoming()
        .for_each(move |stream| {
            let input_device = input_device.clone();
            let (sink, stream) = stream.split();
            tokio::spawn(
                Sink::new(&input_device)
                    .stream_into(sink)
                    .map(|_| ())
                    .map_err(|_| ()),
            );
            Ok(())
        })
        .map_err(|err| {
            eprintln!("accept error: {}", err);
        });
    */

    // tokio::run(server);
}