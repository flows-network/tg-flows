use tg_flows::{listen_to_update, Telegram, UpdateKind};

#[no_mangle]
pub fn run() {
    let token = std::env::var("TOKEN").unwrap();
    let tele = Telegram::new(token.clone());

    listen_to_update(token, |update| {
        if let UpdateKind::Message(msg) = update.kind {
            let text = msg.text().unwrap_or("???");

            _ = tele.send_message(msg.chat.id, format!("echoing: {text}"));
        }
    });
}
