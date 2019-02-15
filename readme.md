# rust-masstransit #

A play repo to try and integrate MassTransit clients with a rust client.

The `dotnet` directory contains a trivial MT requester, publishing a `Ping` and expecting a `Pong` message back.

The rust `src` contains a handler that connects to the rabbit broker, creates a queue, (doesn't yet) configure bindings for the Ping to it's queue, and then responds to those with a Pong, wrapped in the same envelope of metadata that MassTransit expects.





