use tg_flows::{ChatId, Telegram};

fn main() {
    let token = env!("TOKEN").to_string();
    let tele = Telegram::new(token);

    _ = tele.send_message(ChatId(123456), "bonjour!");
}
