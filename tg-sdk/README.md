<div align="center">
  <h1><code>Telgram Flows</code></h1>
  <a href="https://docs.rs/tg-flows/">
    <img src="https://docs.rs/tg-flows/badge.svg">
  </a>
  <a href="https://crates.io/crates/tg-flows">
    <img src="https://img.shields.io/crates/v/tg-flows.svg">
  </a>

  Telegram Integration for [Flows.network](https://test.flows.network)
</div>

## Quick Start

There is a echo bot, but plain text:

```rust
use tg_flows::{listen_to_update, Telegram, UpdateKind};

#[no_mangle]
pub fn run() {
    let telegram_token = std::env::var("telegram_token").unwrap();
    let tele = Telegram::new(telegram_token.clone());

    listen_to_update(telegram_token, |update| {
        if let UpdateKind::Message(msg) = update.kind {
            let text = msg.text().unwrap_or("");
            let chat_id = msg.chat.id;

            let _sended_msg = tele.send_message(chat_id, text);
        }
    });
}
```

[Telegram::new()] is a `Telegram` constructor that represents a bot.

[listen_to_update()] is responsible for registering a listener for the bot
represented by the `telegram_token`. When a new `Update` coming, the callback
is called with received `Update`.
