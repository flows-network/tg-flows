mod api;
mod types;
mod util;

pub use api::*;
pub use types::*;

pub use tg_flows_macros::*;

use http_req::request;
use lazy_static::lazy_static;

use flowsnet_platform_sdk::write_error_log;

lazy_static! {
    static ref TG_API_PREFIX: String = String::from(
        std::option_env!("TG_API_PREFIX").unwrap_or("https://telegram.flows.network/api")
    );
}

extern "C" {
    // Return the user id of the flows platform
    fn get_flows_user(p: *mut u8) -> i32;

    // Return the flow id
    fn get_flow_id(p: *mut u8) -> i32;

    fn set_output(p: *const u8, len: i32);
    fn set_error_code(code: i16);
}

unsafe fn _get_flows_user() -> String {
    let mut flows_user = Vec::<u8>::with_capacity(100);
    let c = get_flows_user(flows_user.as_mut_ptr());
    flows_user.set_len(c as usize);
    String::from_utf8(flows_user).unwrap()
}

unsafe fn _get_flow_id() -> String {
    let mut flow_id = Vec::<u8>::with_capacity(100);
    let c = get_flow_id(flow_id.as_mut_ptr());
    if c == 0 {
        panic!("Failed to get flow id");
    }
    flow_id.set_len(c as usize);
    String::from_utf8(flow_id).unwrap()
}

/// Create a listener for Telegram bot represented by `token`
///
/// Before creating the listener, this function will revoke previous
/// registered listener of current flow so you don't need to do it manually.
///
pub async fn listen_to_update<T>(token: T)
where
    T: ToString,
{
    unsafe {
        let flows_user = _get_flows_user();
        let flow_id = _get_flow_id();

        let mut writer = Vec::new();
        let res = request::get(
            format!(
                "{}/{flows_user}/{flow_id}/listen?token={}&handler_fn={}",
                TG_API_PREFIX.as_str(),
                urlencoding::encode(&token.to_string()),
                "__telegram__on_updated"
            ),
            &mut writer,
        )
        .unwrap();

        match res.status_code().is_success() {
            true => {
                let output = format!(
                    "[{}] Listening for all messages to your bot.",
                    std::env!("CARGO_CRATE_NAME")
                );
                set_output(output.as_ptr(), output.len() as i32);
            }
            false => {
                write_error_log!(String::from_utf8_lossy(&writer));
                set_error_code(format!("{}", res.status_code()).parse::<i16>().unwrap_or(0));
            }
        }
    }
}
