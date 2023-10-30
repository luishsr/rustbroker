use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use tokio::sync::{ mpsc};
use warp::{self, Filter};
use tokio::sync::mpsc::UnboundedSender;
use warp::ws::{Message};
use futures_util::{SinkExt, StreamExt};

type Topic = String;
type Event = String;
type WsSender = UnboundedSender<warp::ws::Message>;

struct Broker {
    events: Arc<RwLock<HashMap<Topic, VecDeque<Event>>>>,
    subscribers: Arc<RwLock<HashMap<Topic, Vec<WsSender>>>>,
}

impl Broker {
    fn new() -> Self {
        Broker {
            events: Arc::new(RwLock::new(HashMap::new())),
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn produce(&self, topic: Topic, event: Event) {
        let mut events = self.events.write().unwrap();
        events.entry(topic.clone()).or_insert_with(VecDeque::new).push_back(event.clone());

        // Notify all subscribers asynchronously.
        let subscribers_list;
        {
            let subscribers = self.subscribers.read().unwrap();
            subscribers_list = subscribers.get(&topic).cloned().unwrap_or_default();
        }

        for ws_sender in subscribers_list {
            // Attempt to send the event to the WebSocket client.
            let _ = ws_sender.send(warp::ws::Message::text(event.clone()));
        }
    }

    pub fn subscribe(&self, topic: Topic, socket: warp::ws::WebSocket) {
        let (ws_sender, mut ws_receiver) = socket.split();

        let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

        {
            let mut subs = self.subscribers.write().unwrap();
            subs.entry(topic).or_insert_with(Vec::new).push(tx);
        }

        tokio::task::spawn(async move {
            while let Some(Ok(_message)) = ws_receiver.next().await {
                // Here you can handle incoming messages from clients if needed
                // For now, it does nothing with incoming messages.
            }
        });

        tokio::task::spawn(async move {
            let mut sender = ws_sender;
            while let Some(msg) = rx.recv().await {
                let _ = sender.send(msg).await;
            }
        });
    }
}

#[tokio::main]
async fn main() {
    let broker = Arc::new(Broker::new());
    let broker_clone1 = Arc::clone(&broker);
    let broker_clone2 = Arc::clone(&broker);

    let produce = warp::path!("produce" / String)
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&broker_clone1)))
        .and_then(move |topic: String, event: Event, broker_clone2: Arc<Broker>| {
            async move {
                broker_clone2.produce(topic, event).await;
                Ok::<_, warp::Rejection>(warp::reply())
            }
        });

    let subscribe = warp::path!("subscribe" / String)
        .and(warp::ws())
        .map(move |topic: String, ws: warp::ws::Ws| {
            let broker_clone3 = Arc::clone(&broker_clone2);
            ws.on_upgrade(move |socket| {
                async move {
                    broker_clone3.subscribe(topic.clone(), socket);
                }
            })
        });

    let routes = produce.or(subscribe);

    println!("Broker server running at http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_produce() {
        let broker = Broker::new();

        broker.produce("test_topic".to_string(), "test_message".to_string()).await;

        let events = broker.events.read().unwrap();
        let messages = events.get("test_topic").unwrap();

        assert_eq!(messages.front().unwrap().as_str(), "test_message");
    }

    #[tokio::test]
    async fn test_warp_subscribe() {
        let broker = Arc::new(Broker::new());

        let subscribe = warp::path!("subscribe" / String)
            .and(warp::ws())
            .map(move |topic: String, ws: warp::ws::Ws| {
                let broker_clone = Arc::clone(&broker);
                ws.on_upgrade(move |socket| {
                    async move {
                        broker_clone.subscribe(topic.clone(), socket);
                    }
                })
            });

        // Test our filter
        let result = warp::test::request()
            .method("GET")
            .path("/subscribe/test_topic")
            .header("sec-websocket-key", "some_random_key")
            .header("Sec-websocket-version", "13")
            .header("upgrade", "websocket")
            .header("connection", "upgrade")
            .filter(&subscribe)
            .await;

        // Print out more detailed information if the result is an Err
        if let Err(e) = &result {
            println!("Error encountered: {:?}", e);
        }

        assert!(result.is_ok());

    }

}
