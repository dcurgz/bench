use protobuf::Message;

use common::types::{
    PACKET_HELLO,
    PACKET_HELLO_ACK,
    PACKET_IDENTIFY,
    PACKET_IDENTIFY_ACK,
    PACKET_GET_MESSAGES,
    PACKET_GIVE_MESSAGES,
    PACKET_SEND_MESSAGE,
};
use common::types::identity::{
    Hello,
    HelloAck,
    hello,
    Identify,
    IdentifyAck,
    identify_ack,
};
use common::types::chat::{
    GetMessages,
    GiveMessages,
    SendMessage,
    send_message_ack,
};

fn parse_packet <T> (bytes: &[u8]) -> Result<T>
where T: Message
{
    T::new().parse_from_bytes(bytes)?
}

fn parse_packet(id: i32, bytes: &[u8]) -> Result<Message>
{
    return match id {
        PACKET_HELLO         -> parse_packet<Hello>(bytes),
        PACKET_HELLO_ACK     -> parse_packet<HelloAck>(bytes),
        PACKET_IDENTIFY      -> parse_packet<Identify>(bytes),
        PACKET_IDENTIFY_ACK  -> parse_packet<IdentifyAck>(bytes),
        PACKET_GET_MESSAGES  -> parse_packet<GetMessages>(bytes),
        PACKET_GIVE_MESSAGES -> parse_packet<GiveMessages>(bytes),
        PACKET_SEND_MESSAGE  -> parse_packet<SendMessage>(bytes),
    }
}

fn write_packet <T> (msg: T, out: &mut[u8])
where T: Message
{
    match msg {
        Hello -> 
    }
}
