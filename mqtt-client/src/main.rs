use rumqttc::{Client, MqttOptions, QoS};
use serde::Serialize;
use std::time::Duration;

#[derive(Serialize)]
struct Payload {
    msg: String,
    value: i32,
}

fn main() {
    let mut options = MqttOptions::new("rust-client", "mosquitto", 1883);
    options.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut connection) = Client::new(options, 10);

    client.subscribe("test/topic", QoS::AtLeastOnce).unwrap();

    let payload = Payload {
        msg: "Hello from Rust".into(),
        value: 1,
    };

    let json = serde_json::to_string(&payload).unwrap();
    client.publish("test/topic", QoS::AtLeastOnce, false, json).unwrap();

    for event in connection.iter() {
        println!("Received = {:?}", event);
    }
}
