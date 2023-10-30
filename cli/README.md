# CLI WebSocket Broker

A simple WebSocket-based message broker allowing clients to produce and consume messages via WebSocket connections. Built with Rust and the Warp framework.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Broker](#broker)
  - [CLI](#cli)
- [Tests](#tests)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Produce**: Send messages to a specified topic.
- **Subscribe**: Listen to messages on a given topic via WebSocket.

## Installation

1. Ensure you have Rust and Cargo installed on your machine. If not, install them from [here](https://www.rust-lang.org/tools/install).
2. Clone the repository:

       git clone https://github.com/yourusername/cli-websocket-broker.git
       cd cli-websocket-broker

3. Build the project:
   
       cargo build --release

Usage

Broker

First, run the WebSocket broker:

    cargo run

This starts the broker on 127.0.0.1:3030.

CLI

Producing a Message:

    cargo run produce <TOPIC> <MESSAGE>

Subscribing to a Topic:

    cargo run subscribe <TOPIC>

Tests

To run the tests:

    cargo test

