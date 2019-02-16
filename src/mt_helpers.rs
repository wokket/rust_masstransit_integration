//! Helper Methods for working with MassTransit
use super::messages::MassTransitMessageEnvelope;
use amqp::{protocol, Basic, Channel};

/// Converts a Mass Transit URN (eg rabbitmq://localhost/bus-TANGO3-dotnet-hkhoyygh4csfpdzdbdmjgn3a8x?durable=false&autodelete=true ) into
/// an exchange name that we can use in AMPQ-land (eg bus-TANGO3-dotnet-hkhoyygh4csfpdzdbdmjgn3a8x)
pub fn convert_urn_to_exchange<'a>(mt_urn: &'a str) -> &'a str {
    let last_slash = match mt_urn.rfind('/') {
        None => 0,
        Some(x) => x + 1,
    };

    let question_mark = match mt_urn.rfind('?') {
        None => mt_urn.len(),
        Some(x) => x,
    };

    &mt_urn[last_slash..question_mark]
}

/// Performs sanity checking of the message data prior  to attempting to process it.
/// message_type: The type (namespace? FQN?) of the .Net message we're expecting this AMQP message to contain.  
/// If the .Net type is `Namespace.TypeName`, this should normally be `Namespace:TypeName` because MT does crazy things.
/// You can confirm this value by looking at the Exchanges created by MT in Rabbit
pub fn perform_sanity_checks(
    message_type: &str,
    deliver: &protocol::basic::Deliver,
    headers: &protocol::basic::BasicProperties,
) {
    // debug info
    //eprintln!("Deliver info: {:?}", deliver);
    //eprintln!("Content headers: {:?}", headers);

    // Ensure we've come from a correctly-typed exchange
    assert_eq!(deliver.exchange, message_type, "Incorrect message type?");

    //ensure we have JSON data
    assert_eq!(
        headers.content_type,
        Some("application/vnd.masstransit+json".to_string()),
        "This crate requires a json encoded MassTransit message!"
    );
}

pub fn send_reply<T>(
    channel: &mut Channel,
    dest_addr: String,
    envelope: MassTransitMessageEnvelope<T>,
) where
    T: std::fmt::Debug,
    T: serde::Serialize,
{
    // publish
    channel
        .basic_publish(
            //exchange
            convert_urn_to_exchange(&dest_addr), //exchange
            "",                                  //route key
            true,                                //mandatory
            false,                               //immediate
            protocol::basic::BasicProperties {
                //TODO: Any more MT headers we need here?
                content_type: Some("application/vnd.masstransit+json".to_string()),
                ..Default::default()
            },
            serde_json::to_string(&envelope)
                .unwrap()
                .as_bytes()
                .to_vec(),
        )
        .ok()
        .expect("Failed to publish!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_urn() {
        let input = "rabbitmq://localhost/bus-TANGO3-dotnet-hkhoyygh4csfpdzdbdmjgn3a8x?durable=false&autodelete=true";
        let expected = "bus-TANGO3-dotnet-hkhoyygh4csfpdzdbdmjgn3a8x";

        assert_eq!(expected, convert_urn_to_exchange(input));
    }
}
