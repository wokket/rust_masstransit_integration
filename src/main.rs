extern crate amqp;
extern crate chrono;
extern crate env_logger;
extern crate serde_json;
extern crate uuid;

mod messages;
mod mt_helpers;

use amqp::QueueBuilder;
use amqp::TableEntry::LongString;
use amqp::{Basic, Options, Session, Table};
use std::default::Default;

fn main() {
    env_logger::init();

    let mut props = Table::new();
    props.insert("example-name".to_owned(), LongString("consumer".to_owned()));

    let mut session = Session::new(Options {
        properties: props,
        vhost: "/".to_string(),
        ..Default::default()
    })
    .ok()
    .expect("Can't create session");

    let mut channel = session
        .open_channel(1)
        .ok()
        .expect("Error openning channel 1");
    println!("Opened channel: {:?}", channel.id);

    let queue_name = "test_queue";
    let queue_builder = QueueBuilder::named(queue_name).durable();
    let queue_declare = queue_builder.declare(&mut channel);

    println!("Queue declare: {:?}", queue_declare);
    channel.basic_prefetch(10).ok().expect("Failed to prefetch");
    //consumer, queue: &str, consumer_tag: &str, no_local: bool, no_ack: bool, exclusive: bool, nowait: bool, arguments: Table
    println!("Declaring consumers...");

    // let consume_builder = ConsumeBuilder::new(consumer_function, queue_name);
    // let consumer_name = consume_builder.basic_consume(&mut channel);
    // println!("Starting consumer {:?}", consumer_name);

    let my_consumer = messages::Ping {};

    let consumer_name = channel.basic_consume(
        my_consumer,
        queue_name,
        "",
        false,
        false,
        false,
        false,
        Table::new(),
    );
    println!("Starting consumer {:?}", consumer_name);

    channel.start_consuming();

    channel.close(200, "Bye").unwrap();
    session.close(200, "Good Bye");
}
