# Rust Event Broker

Rust Event Broker is a lightweight event streaming platform written in Rust that provides topic-based event management, allowing you to produce and consume events.

## Features

- **Topic-Based Event Management:** Organize events into different topics for better categorization.
- **Event Production:** Send events to specific topics.
- **Event Consumption:** Retrieve and consume the latest event from a specific topic.
- **Real-Time Subscriptions:** Subscribe to topics and receive events in real-time using WebSockets.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/)

### Installation

1. Clone this repository:

    ```shell
    git clone https://github.com/yourusername/rust-event-broker.git
    ```

2. Change into the project directory:

    ```shell
    cd rust-event-broker
    ```

3. Build the project:

    ```shell
    cargo build
    ```

4. Run the server:

    ```shell
    cargo run
    ```

### Usage

#### Producing Events

To produce an event to a specific topic, send a POST request with the event data to `http://localhost:3030/produce/<topic>`.

    ```shell
    curl -X POST -H "Content-Type: application/json" -d '{"event": "Event Data"}' http://localhost:3030/produce/<topic>

Consuming Events

To consume the latest event from a specific topic, send a GET request to http://localhost:3030/consume/<topic>.

    ```shell

    curl http://localhost:3030/consume/<topic>

Subscribing to Topics (WebSockets)

You can subscribe to a topic using WebSockets by connecting to ws://localhost:3030/subscribe/<topic>. When events are produced for the subscribed topic, they will be pushed in real-time to the WebSocket client.

Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue. If you'd like to contribute, fork the repository, create a branch, and submit a pull request.
License

This project is licensed under the MIT License - see the LICENSE file for details.
