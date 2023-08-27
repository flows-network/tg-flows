# Telegram Integration for [Flows.network](https://flows.network)

## Quick Start

There is a echo bot, but plain text:

```rust
use tg_flows::{listen_to_update, update_handler, Telegram, UpdateKind};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    let telegram_token = std::env::var("telegram_token").unwrap();
    listen_to_update(telegram_token).await;
}

#[update_handler]
async fn handler(update: tg_flows::Update) {
    let telegram_token = std::env::var("telegram_token").unwrap();
    let tele = Telegram::new(telegram_token);

    if let UpdateKind::Message(msg) = update.kind {
        let text = msg.text().unwrap_or("");
        let chat_id = msg.chat.id;
        let _sended_msg = tele.send_message(chat_id, text);
    }
}
```

[Telegram::new()](https://docs.rs/tg-flows/latest/tg_flows/struct.Telegram.html#method.new) is a `Telegram` constructor that represents a bot.

[listen_to_update()](https://docs.rs/tg-flows/latest/tg_flows/fn.listen_to_update.html) is responsible for registering a listener for the bot
represented by the `telegram_token`. When a new [Update](https://docs.rs/tg-flows/latest/tg_flows/struct.Update.html) coming, the `handler`
fn decorated by [update_handler](https://docs.rs/tg-flows/latest/tg_flows/macro.update_handler.html) is called.

The whole document is [here](https://docs.rs/tg-flows).
