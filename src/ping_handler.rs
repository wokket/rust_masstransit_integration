use super::messages::*;
use super::mt_helpers;
use amqp::{protocol, Basic, Channel};
use chrono::Utc;

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

impl Handler {
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
