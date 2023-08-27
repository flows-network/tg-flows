use tg_flows::{ChatId, Telegram, UserId};

fn main() {
    let token = env!("TOKEN").to_string();
    let tele = Telegram::new(token);

    let chat_id = ChatId(123456);
    let user_id = UserId(123456);

    let result = tele.get_chat_member(chat_id, user_id);

    match result {
        Ok(chat_member) => println!("{:#?}", chat_member),
        Err(e) => eprintln!("{e}"),
    }
}
