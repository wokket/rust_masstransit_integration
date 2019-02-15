// Message schemas that align with the .NEt MassTransit types

use super::mt_helpers;
use amqp::{protocol, Basic, Channel};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Ping {}

#[derive(Serialize, Debug)]
pub struct Pong {
    #[serde(rename = "replyValue")]
    reply_value: String, // same as inbound ping
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MassTransitMessageEnvelope<'a> {
    //from https://github.com/MassTransit/MassTransit/blob/master/src/MassTransit/Serialization/MessageEnvelope.cs
    #[serde(rename = "messageId")]
    message_id: Uuid,

    #[serde(rename = "requestId")]
    request_id: String,

    #[serde(rename = "correlationId")]
    correlation_id: Option<String>,

    #[serde(rename = "conversationId")]
    conversation_id: String,

    #[serde(rename = "sourceAddress")]
    source_address: String,

    #[serde(rename = "destinationAddress")]
    destination_address: &'a str,

    #[serde(rename = "responseAddress")]
    response_address: String,

    #[serde(rename = "faultAddress")]
    fault_address: Option<String>,

    #[serde(rename = "messageType")]
    message_type: Vec<String>,

    message: serde_json::Value, // more json

    #[serde(rename = "expirationTime")]
    expiration_time: Option<String>, //DateTime?

    #[serde(rename = "sentTime")]
    sent_time: Option<DateTime<Utc>>, //DateTime?

    // Headers: Map<String, String>, //TODO: Headers
    host: BusHostInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusHostInfo {
    //from https://github.com/MassTransit/MassTransit/blob/master/src/MassTransit/Util/BusHostInfo.cs
}

impl amqp::Consumer for Ping {
    fn handle_delivery(
        &mut self,
        channel: &mut Channel,
        deliver: protocol::basic::Deliver,
        headers: protocol::basic::BasicProperties,
        body: Vec<u8>,
    ) {
        eprintln!("Deliver info: {:?}", deliver);
        eprintln!("Content headers: {:?}", headers);

        let body_as_string = String::from_utf8(body).expect("Binary message received!");
        let envelope: MassTransitMessageEnvelope = serde_json::from_str(&body_as_string)
            .expect("Unable to deserialise message to MassTransitEnvelope");

        println!("Deserialized  Envelope: {:?}", envelope);

        assert_eq!(deliver.exchange, "Messages:Ping", "Incorrect message type?");
        assert_eq!(
            headers.content_type,
            Some("application/vnd.masstransit+json".to_string()),
            "This crate requires a json encoded MassTransit message!"
        );

        assert_eq!(
            envelope.message_type[0], "urn:message:Messages:Ping",
            "This handler requires a Ping message!" //TODO: Can we automaticaly check the name of self against the MT data for some level of re-usable safety?
        );

        let value = &envelope.message["value"];

        let pong = Pong {
            reply_value: format!("Reply to '{}' from rust!", value),
        };

        self.reply(channel, pong, envelope);

        // DO SOME JOB:
        //self.deliveries_number += 1;
        channel.basic_ack(deliver.delivery_tag, false).unwrap();
    }
}

impl Ping {
    fn reply(
        &self,
        channel: &mut Channel,
        pong: Pong,
        request_envelope: MassTransitMessageEnvelope,
    ) {
        let dest_addr = request_envelope.response_address.clone();

        let sending_envelope = MassTransitMessageEnvelope {
            message_id: Uuid::new_v4(),
            source_address: "Rust_Endpoint".to_string(), //TODO:
            destination_address: &request_envelope.response_address.clone(), //make sure we send the response to teh correct place
            message_type: vec!["urn:message:Messages:Pong".to_string()], //TODO: Can we auto-generate this value somehow?
            message: serde_json::to_value(pong).unwrap(),
            sent_time: Some(Utc::now()),
            ..request_envelope //just reuse all the other fields (conversation id's etc)
        };

        println!("Publishing response: {:?}", sending_envelope);

        // publish
        channel
            .basic_publish(
                //exchange
                mt_helpers::convert_urn_to_exchange(&dest_addr), //exchange
                "",                                              //route key
                true,                                            //mandatory
                false,                                           //immediate
                protocol::basic::BasicProperties {
                    content_type: Some("application/vnd.masstransit+json".to_string()),
                    ..Default::default()
                },
                serde_json::to_string(&sending_envelope)
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
            )
            .ok()
            .expect("Failed to publish!");
    }
}
