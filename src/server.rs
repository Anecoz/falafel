mod common;

//use std::sync::mpsc;
use std::thread;
use std::io::{self, BufReader, BufWriter, Read};
use std::net::{TcpListener, TcpStream};

fn read_packet(reader: &mut BufReader<&TcpStream>) -> Result<common::packet::PacketType, io::Error> {
  let mut buf: Vec<u8> = vec![0; common::packet::HEADER_SIZE as usize];
  match reader.read_exact(&mut buf) {
    Ok(_) => (),
    Err(e) => return Err(e),
  }
  let header = common::packet::deserialize_header(buf);

  let mut buf: Vec<u8> = vec![0; header.packet_size as usize];
  match reader.read_exact(&mut buf) {
    Ok(_) => (),
    Err(e) => return Err(e),
  }
  Ok(common::packet::deserialize(header, buf))
}

fn handle_new_connection(stream: TcpStream) {
  thread::spawn(move || {
    let mut reader = BufReader::new(&stream);
    let mut _writer = BufWriter::new(&stream);

    // Expect a connection message to be received.
    let packet = match read_packet(&mut reader) {
      Ok(packet) => packet,
      Err(_) => {
        println!("Dropping connection immediately, could not read connection header.");
        return;
      },
    };
    let mut connection_name = String::from("unknown");

    if let common::packet::PacketType::Connection(name) = packet {
      println!("Received new connection: {}", name);
      connection_name = name;
    }
   
    loop {
      let packet = match read_packet(&mut reader) {
        Ok(packet) => packet,
        Err(_) => {
          println!("{} disconnected.", connection_name);
          break;
        },
      };

      if let common::packet::PacketType::Message(msg) = packet {
        println!("Received from {}: {}", connection_name, msg);
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
