use super::messages::*;
use super::mt_helpers;
use amqp::{protocol, Basic, Channel};
use chrono::Utc;
use std::fmt::Debug;
use uuid::Uuid;

pub struct Handler();

impl amqp::Consumer for Handler {
    fn handle_delivery(
        &mut self,
        channel: &mut Channel,
        deliver: protocol::basic::Deliver,
        headers: protocol::basic::BasicProperties,
        body: Vec<u8>,
    ) {
        mt_helpers::perform_sanity_checks("Messages:Ping", &deliver, &headers);

        let body_as_string =
            String::from_utf8(body).expect("Binary or invalid message received from AMQP!");

        let envelope: MassTransitMessageEnvelope<Ping> = serde_json::from_str(&body_as_string)
            .expect("Unable to deserialise AMQP message body to MassTransitEnvelope");

        // eprintln!("Deserialized  Envelope: {:?}", envelope);

        assert_eq!(
            envelope.message_type[0], "urn:message:Messages:Ping",
            "This handler requires a Ping message!" //TODO: Can we automatically check the name of self against the MT data for some level of re-usable safety?
        );

        /////////////////////////////
        // Do our actual work, in this case just build a reply message

        let value = envelope.message.value.clone();

        let pong = Pong {
            reply_value: format!("Reply to '{}' from rust!", value),
        };

        /////////////////////////////

        self.reply(channel, pong, envelope);

        channel.basic_ack(deliver.delivery_tag, false).unwrap();
    }
}

impl Handler {
    fn reply<REQ, RESP>(
        &self,
        channel: &mut Channel,
        reply: RESP,
        request_envelope: MassTransitMessageEnvelope<REQ>,
    ) where
        REQ: Debug,
        RESP: Debug,
        RESP: serde::Serialize,
    {
        let dest_addr = request_envelope.response_address.unwrap().clone();

        let sending_envelope = MassTransitMessageEnvelope {
            message_id: Uuid::new_v4(),
            source_address: "Rust_Endpoint".to_string(), //TODO:
            destination_address: dest_addr.clone(), //make sure we send the response to teh correct place
            message_type: vec!["urn:message:Messages:Pong".to_string()], //TODO: Can we auto-generate this value somehow?
            message: reply,
            sent_time: Some(Utc::now()),
            conversation_id: request_envelope.conversation_id,
            correlation_id: request_envelope.correlation_id,
            expiration_time: request_envelope.expiration_time,
            fault_address: request_envelope.fault_address,
            request_id: request_envelope.request_id,
            response_address: None,
            host: request_envelope.host,
            // we can't just reuse the inbound fields now we have differing generic types
            //..request_envelope //just reuse all the other fields (conversation id's etc)
        };

        mt_helpers::send_reply(channel, dest_addr, sending_envelope);
    }
}
