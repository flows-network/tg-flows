use tg_flows::Telegram;

fn main() {
    let token = env!("TOKEN").to_string();
    let tele = Telegram::new(token);

    let result = tele.get_me();

    match result {
        Ok(me) => println!("{:#?}", me),
        Err(e) => eprintln!("{e}"),
    }
}
