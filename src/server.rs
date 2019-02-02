mod common;

//use std::sync::mpsc;
use std::thread;
use std::io::{BufReader, BufWriter, Write, BufRead, Read};
use std::net::{TcpListener, TcpStream};

fn handle_new_connection(stream: TcpStream) {
  thread::spawn(move || {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    // Greet the new connection with a welcome message.
    writer.write(b"Welcome to Uganda, friend!\n").expect("Could not write!");
    writer.flush().expect("Could not flush!");

    // Loop and wait for the connection's messages.    
    loop {
      let mut buf: Vec<u8> = vec![0; common::packet::HEADER_SIZE as usize];
      let result = reader.read_exact(&mut buf).expect("Could not read header!");
      let header = common::packet::deserialize_header(buf);

      let mut buf: Vec<u8> = vec![0; header.packet_size as usize];
      let content_result = reader.read_exact(&mut buf).expect("Could not read content!");
      let packet = common::packet::deserialize(header, buf);

      if let common::packet::PacketType::Message(msg) = packet {
        print!("Received from connection: {}", msg);
      }
    }
  });
}

fn main() {
  println!("Starting server on port 8888");

  let listener = match TcpListener::bind("127.0.0.1:8888") {
    Ok(listener) => listener,
    Err(error) => panic!(error),
  };

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        println!("Received new connection.");
        handle_new_connection(stream);
      },
      Err(e) => panic!("Received error while accepting connections: {}", e),
    }
  }

  /*let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi!");
    match tx.send(val) {
      Ok(()) => (),
      Err(string) => println!("Send error: {}", string),
    }
  });

  for r in rx {
    println!("Received: {}", r);
  }*/
}
