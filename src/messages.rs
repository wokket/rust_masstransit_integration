// Message schemas that align with the .NEt MassTransit types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Ping {
    pub value: String,
}

#[derive(Serialize, Debug)]
pub struct Pong {
    #[serde(rename = "replyValue")]
    pub reply_value: String, // same as inbound ping
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MassTransitMessageEnvelope<'a> {
    //from https://github.com/MassTransit/MassTransit/blob/master/src/MassTransit/Serialization/MessageEnvelope.cs
    #[serde(rename = "messageId")]
    pub message_id: Uuid,

    #[serde(rename = "requestId")]
    pub request_id: String,

    #[serde(rename = "correlationId")]
    pub correlation_id: Option<String>,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "sourceAddress")]
    pub source_address: String,

    #[serde(rename = "destinationAddress")]
    pub destination_address: &'a str,

    #[serde(rename = "responseAddress")]
    pub response_address: String,

    #[serde(rename = "faultAddress")]
    pub fault_address: Option<String>,

    #[serde(rename = "messageType")]
    pub message_type: Vec<String>,

    pub message: serde_json::Value, // more json

    #[serde(rename = "expirationTime")]
    pub expiration_time: Option<String>, //DateTime?

    #[serde(rename = "sentTime")]
    pub sent_time: Option<DateTime<Utc>>, //DateTime?

    // Headers: Map<String, String>, //TODO: Headers
    pub host: BusHostInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusHostInfo {
    //from https://github.com/MassTransit/MassTransit/blob/master/src/MassTransit/Util/BusHostInfo.cs
}
