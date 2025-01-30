![Slack Logo](https://upload.wikimedia.org/wikipedia/commons/7/76/Slack_Icon.png)

`rust_slack_sdk` is a Rust crate that provides an easy-to-use interface for integrating Slack with Rust applications. It supports sending messages, handling events, managing users and channels, and working with files.

## Features
- **Authentication**: Supports OAuth and bot token authentication.
- **Messaging**: Send and read messages from Slack channels.
- **Event Handling**: Subscribe to Slack events and process them.
- **User Management**: Fetch user details and presence.
- **Channel Management**: List, create, and manage Slack channels.
- **File Uploads**: Upload and retrieve files from Slack.
- **Utilities**: Built-in rate-limiting and retry mechanisms.

## Installation
Add `rust_slack_sdk` to your `Cargo.toml`:

```toml
[dependencies]
rust_slack_sdk = "0.1"
```

## Usage

### Authentication

```rust
use rust_slack_sdk::SlackClient;

let client = SlackClient::new("your-slack-bot-token");
```

### Sending a Message

```rust
use rust_slack_sdk::Messaging;

client.send_message("#general", "Hello, Slack!").unwrap();
```

### Listening to Events

```rust
use rust_slack_sdk::Events;

client.listen_events(|event| {
    println!("Received event: {:?}", event);
});
```

### Fetching Users

```rust
use rust_slack_sdk::Users;

let users = client.get_users().unwrap();
println!("Users: {:?}", users);
```

## Roadmap
- [ ] Support for Slack Slash Commands
- [ ] Interactive Message Handling
- [x] Enhanced Error Handling
- [x] Async Support

## Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License.