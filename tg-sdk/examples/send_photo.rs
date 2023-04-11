use tg_flows::{ChatId, InputFile, Telegram};
use url::Url;

fn main() {
    let token = env!("TOKEN").to_string();
    let tele = Telegram::new(token);

    let crustaceans = "https://rustacean.net/assets/rustacean-orig-noshadow.png";
    let url = Url::try_from(crustaceans).unwrap();

    _ = tele.send_photo(ChatId(852478443), InputFile::url(url));
}
