mod common;

use std::net::TcpStream;
use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::thread;

fn handle_receiving(stream: TcpStream)
{
  thread::spawn(move || {
    let mut reader = BufReader::new(&stream);
    let mut line = String::with_capacity(512);
    loop {
      let result = reader.read_line(&mut line);
      match result {
        Ok(_) => {
          println!("Received from server: {}", line);
        },
        Err(e) => panic!("Could not read to string {}", e),
      }
    }
  });
}

fn main() {
  let stream = match TcpStream::connect("127.0.0.1:8888") {
    Ok(stream) => stream,
    Err(e) => panic!("Could not connect: {}", e),
  };

  handle_receiving(stream.try_clone().expect("Could not clone stream"));

  println!("Connected.");

  let mut writer = BufWriter::new(&stream);  
  loop {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");

    let message = common::packet::PacketType::Message(line);
    let serialized = common::packet::serialize(message);

    writer.write(&serialized[..]).expect("Could not write line to stream");
    writer.flush().expect("Could not flush");
  }
}