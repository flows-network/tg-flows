use http_req::request;
use lazy_static::lazy_static;
use tg_flows::Update;

lazy_static! {
    static ref TG_API_PREFIX: String = String::from(
        std::option_env!("TG_API_PREFIX").unwrap_or("https://telegram.flows.network/api")
    );
}

extern "C" {
    fn get_event_body_length() -> i32;
    fn get_event_body(p: *mut u8) -> i32;
    fn get_event_headers_length() -> i32;
    fn get_event_headers(p: *mut u8) -> i32;
    fn set_flows(p: *const u8, len: i32);
}

#[no_mangle]
pub unsafe fn message() {
    if let Some(_) = update_from_subcription() {
        let headers = headers_from_subcription().unwrap_or_default();
        let token = headers
            .into_iter()
            .find(|header| header.0.to_ascii_lowercase() == "x-telegram-bot-api-secret-token")
            .unwrap_or((String::new(), String::new()))
            .1;

        let mut writer = Vec::new();
        let res = request::get(
            format!("{}/event/{}", TG_API_PREFIX.as_str(), token),
            &mut writer,
        )
        .unwrap();

        if res.status_code().is_success() {
            if let Ok(flows) = String::from_utf8(writer) {
                set_flows(flows.as_ptr(), flows.len() as i32);
            }
        }
    }
}

fn update_from_subcription() -> Option<Update> {
    unsafe {
        let l = get_event_body_length();
        let mut event_body = Vec::<u8>::with_capacity(l as usize);
        let c = get_event_body(event_body.as_mut_ptr());
        assert!(c == l);
        event_body.set_len(c as usize);

        match serde_json::from_slice(&event_body) {
            Ok(e) => Some(e),
            Err(_) => None,
        }
    }
}

fn headers_from_subcription() -> Option<Vec<(String, String)>> {
    unsafe {
        let l = get_event_headers_length();
        let mut event_body = Vec::<u8>::with_capacity(l as usize);
        let c = get_event_headers(event_body.as_mut_ptr());
        assert!(c == l);
        event_body.set_len(c as usize);

        match serde_json::from_slice(&event_body) {
            Ok(e) => Some(e),
            Err(_) => None,
        }
    }
}
