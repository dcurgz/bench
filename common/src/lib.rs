pub mod types {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
 
    use std::fmt;
    use std::error::Error;

    use crate::types::identity::{
        Hello,
        HelloAck,
        Identify,
        IdentifyAck,
    };
    use crate::types::chat::{
        GetMessages,
        GiveMessages,
        SendMessage,
    };

    use protobuf::Message;

    pub const PACKET_HELLO: i32 = 0x01;
    pub const PACKET_HELLO_ACK: i32 = 0x02;
    pub const PACKET_IDENTIFY: i32 = 0x03;
    pub const PACKET_IDENTIFY_ACK: i32 = 0x04;
    pub const PACKET_GET_MESSAGES: i32 = 0x05;
    pub const PACKET_GIVE_MESSAGES: i32 = 0x06;
    pub const PACKET_SEND_MESSAGE: i32 = 0x07;

    enum Packet {
        Hello(Hello),
        HelloAck(HelloAck),
        Identify(Identify),
        IdentifyAck(IdentifyAck),
        GetMessages(GetMessages),
        GiveMessages(GiveMessages),
        SendMessage(SendMessage),
    }

    #[derive(Debug)]
    struct ParsePacketError(i32);
    impl fmt::Display for ParsePacketError {
         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
             write!(f, "failed to parse packet from id {0}", self.0)
         }
    }
    impl Error for ParsePacketError { }

    trait HasId {
        fn id(&self) -> i32;
    }

    trait Parse {
        fn parse(id: i32, bytes: &[u8]) -> Result<Self, ParsePacketError>
        where Self: Sized;
    }

    impl Parse for Packet {
        fn parse(id: i32, bytes: &[u8]) -> Result<Self, ParsePacketError> {
            match id {
                PACKET_HELLO =>
                    Ok(Packet::Hello(Hello::parse_from_bytes(bytes).unwrap())),
                PACKET_HELLO_ACK =>
                    Ok(Packet::Hello(Hello::parse_from_bytes(bytes).unwrap())),
                PACKET_IDENTIFY =>
                    Ok(Packet::Identify(Identify::parse_from_bytes(bytes).unwrap())),
                PACKET_IDENTIFY_ACK =>
                    Ok(Packet::IdentifyAck(IdentifyAck::parse_from_bytes(bytes).unwrap())),
                PACKET_GET_MESSAGES =>
                    Ok(Packet::GetMessages(GetMessages::parse_from_bytes(bytes).unwrap())),
                PACKET_GIVE_MESSAGES =>
                    Ok(Packet::GiveMessages(GiveMessages::parse_from_bytes(bytes).unwrap())),
                PACKET_SEND_MESSAGE =>
                    Ok(Packet::SendMessage(SendMessage::parse_from_bytes(bytes).unwrap())),
                _ =>
                    Err(ParsePacketError(id)),
            }
        }
    }

    impl HasId for Hello {
        fn id(&self) -> i32 {
            PACKET_HELLO
        }
    }

    impl HasId for HelloAck {
        fn id(&self) -> i32 {
            PACKET_HELLO_ACK
        }
    }

    impl HasId for Identify {
        fn id(&self) -> i32 {
            PACKET_IDENTIFY
        }
    }

    impl HasId for IdentifyAck {
        fn id(&self) -> i32 {
            PACKET_IDENTIFY_ACK
        }
    }

    impl HasId for GetMessages {
        fn id(&self) -> i32 {
            PACKET_GET_MESSAGES
        }
    }

    impl HasId for GiveMessages {
        fn id(&self) -> i32 {
            PACKET_GIVE_MESSAGES
        }
    }

    impl HasId for SendMessage {
        fn id(&self) -> i32 {
            PACKET_SEND_MESSAGE
        }
    }
}
