pub mod packet {
  pub const HEADER_SIZE: u32 = 2; // 1 byte for type, 1 byte for size of packet

  pub enum PacketType {
    Connection(String), // name in string
    Message(String),
  }

  pub struct PacketHeader {
    pub packet_type: u8,
    pub packet_size: u8,    
  }

  pub fn serialize(packet: PacketType) -> Vec<u8> {
    match packet {
      PacketType::Connection(name) => {
        let mut v: Vec<u8> = Vec::new();
        v.push(0);
        v.push(name.len() as u8);
        v.append(&mut name.as_bytes().to_vec());
        v
      },
      PacketType::Message(msg) => {
        let mut v: Vec<u8> = Vec::new();
        v.push(1);
        v.push(msg.len() as u8);
        v.append(&mut msg.as_bytes().to_vec());
        v
      }
    }
  }

  pub fn deserialize_header(header_bytes: Vec<u8>) -> PacketHeader {
    PacketHeader {
      packet_type: header_bytes[0],
      packet_size: header_bytes[1],
    }
  }

  pub fn deserialize(header: PacketHeader, content_bytes: Vec<u8>) -> PacketType {
    if header.packet_type == 0 {
      let name = String::from_utf8_lossy(&content_bytes);
      PacketType::Connection(name.to_string())
    }
    else {
      let message = String::from_utf8_lossy(&content_bytes);
      PacketType::Message(message.to_string())
    }
  }
}