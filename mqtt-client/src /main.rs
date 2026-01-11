use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde_json::json;  // ✅ Import the json! macro
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mqttoptions = MqttOptions::new("rust-client", "mosquitto", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // Wait for broker to be ready (simple retry)
    for attempt in 1..=5 {
        match client.subscribe("test/topic", QoS::AtMostOnce).await {
            Ok(_) => {
                println!("✅ Connected to broker on attempt {}", attempt);
                break;
            }
            Err(e) => {
                eprintln!("⚠️  Attempt {} failed: {}. Retrying in 2s...", attempt, e);
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }

    // Publish a message
    let payload = json!({
        "message": "Hello from Rust!",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }).to_string();  // ✅ Convert JSON to string

    client.publish("test/topic", QoS::AtLeastOnce, false, payload).await?;  // ✅ Use 'payload', not 'payload_json'

    // Listen for incoming messages
    println!("Listening for MQTT messages...");
    loop {
        match eventloop.poll().await {
            Ok(notification) => {
                println!("Received: {:?}", notification);
            }
            Err(e) => {
                eprintln!("MQTT Error: {}", e);
                break;
            }
        }
    }

    Ok(())
}
