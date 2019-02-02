//use std::sync::mpsc;
use std::thread;
use std::io::{BufReader, BufWriter, Write, BufRead};
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
      let mut line = String::new();
      let result = reader.read_line(&mut line);
      match result {
        Ok(_) => (),
        Err(e) => panic!("Could not read to string {}", e),
      }

      if line.len() > 0 {
        print!("Received from connection: {}", line);
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
