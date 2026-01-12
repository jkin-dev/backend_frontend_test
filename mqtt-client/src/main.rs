use rumqttc::{Client, MqttOptions, QoS};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use log::info;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Payload {
    msg: String,
    value: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut mqttopts = MqttOptions::new("rust-poc-client", "localhost", 1883);
    mqttopts.set_keep_alive(Duration::from_secs(5));

    let (client, _eventloop) = Client::new(mqttopts, 10);

    let payload = Payload {
        msg: "hello from rust".into(),
        value: 42,
    };

    info!("MQTT loop started – publishing every 5 s");

    loop {
        client.publish(
            "test/metrics",
            QoS::AtMostOnce,
            false,
            serde_json::to_string(&payload)?,
        )?;
        info!("Published: {:?}", payload);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
