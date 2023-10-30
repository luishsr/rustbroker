use tungstenite::protocol::Message;
use url::Url;
use clap::{App, Arg, SubCommand};

#[tokio::main]
async fn main() {
    let matches = App::new("Broker CLI")
        .subcommand(SubCommand::with_name("produce")
            .about("Produce an event to a topic")
            .arg(Arg::with_name("topic")
                .help("Topic to produce to")
                .required(true))
            .arg(Arg::with_name("event")
                .help("Event to send")
                .required(true)))
        .subcommand(SubCommand::with_name("subscribe")
            .about("Subscribe to a topic")
            .arg(Arg::with_name("topic")
                .help("Topic to subscribe to")
                .required(true)))
        .get_matches();

    match matches.subcommand() {
        ("produce", Some(produce_matches)) => {
            let topic = produce_matches.value_of("topic").unwrap().to_string();
            let event = produce_matches.value_of("event").unwrap().to_string();
            produce(topic, event).await;
        },
        ("subscribe", Some(subscribe_matches)) => {
            let topic = subscribe_matches.value_of("topic").unwrap().to_string();
            subscribe(topic).await;
        },
        _ => {
            println!("Invalid command");
        }
    }
}

async fn produce(topic: String, event: String) {
    let url = format!("http://127.0.0.1:3030/produce/{}", topic);
    let client = reqwest::Client::new();
    let res = client.post(&url)
        .json(&event)
        .send()
        .await;

    match res {
        Ok(response) if response.status().is_success() => {
            println!("Successfully produced event.");
        },
        Ok(response) => {
            println!("Error producing event: {}", response.status());
        },
        Err(e) => {
            println!("Failed to produce event: {:?}", e);
        }
    }
}

async fn subscribe(topic: String) {
    let url = Url::parse(&format!("ws://127.0.0.1:3030/subscribe/{}", topic)).unwrap();
    let (mut socket, _) = tungstenite::connect(url).expect("Error connecting");

    println!("Connected to the '{}' topic. Waiting for messages...", topic);

    loop {
        let msg = socket.read_message().expect("Error reading message");
        if let Message::Text(txt) = msg {
            println!("Received: {}", txt);
        }
    }
}
