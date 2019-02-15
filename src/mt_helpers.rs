/// Helper Methods for working with MassTransit

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
