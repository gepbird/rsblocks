use crate::config::CONFIG;
use crate::types::ThreadsData;

pub fn get_pub_ip() -> ThreadsData {
    let url = format!("http://api.ipify.org");
    let _err = String::from("Error");
    let res = match minreq::get(url).send() {
        Ok(resp) => match resp.as_str() {
            Ok(res_str) => res_str.trim().to_string(),
            Err(_) => _err,
        },
        Err(_) => _err,
    };

    let data = format!("  {}  {}  {}", CONFIG.pub_ip.icon, res, CONFIG.seperator);
    ThreadsData::PubIp(data)
}
