//! Inter thread protocol

pub struct ChannelMessage {
    pub message_type: MessageTypes,
    pub payload: String,
    pub payload_int: u32,
}

pub enum MessageTypes {
    SerialStart,
    SerialStop,
    SerialSend,
    SerialPort,
    SimStart,
    SimStop,
    SimSendEvent,
}
