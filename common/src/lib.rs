pub mod types {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

    pub const PACKET_HELLO: i32 = 0x01;
    pub const PACKET_HELLO_ACK: i32 = 0x02;
    pub const PACKET_IDENTIFY: i32 = 0x03;
    pub const PACKET_IDENTIFY_ACK: i32 = 0x04;
    pub const PACKET_GET_MESSAGES: i32 = 0x05;
    pub const PACKET_GIVE_MESSAGES: i32 = 0x06;
    pub const PACKET_SEND_MESSAGE: i32 = 0x07;
}
