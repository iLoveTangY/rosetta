use nanoid::nanoid;
use reqwest::blocking::Client;

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct QueryParams {
    q: String,
    from: String,
    to: String,
    appid: String,
    salt: String,
    sign: String,
}

#[derive(Deserialize, Debug)]
struct TranslateResult {
    dst: String,
}

#[derive(Deserialize, Debug)]
struct TranslateResults {
    from: String,
    to: String,
    trans_result: Vec<TranslateResult>,
}

pub async fn translate(text: &str, from: &str, to: &str) {
    let appid = "";
    let secret = "";

    let url = "https://fanyi-api.baidu.com/api/trans/vip/translate";

    let salt = nanoid!();

    let str = format!("{}{}{}{}", appid, text, salt, secret);
    let sign = md5::compute(str);

    let query_params = QueryParams {
        q: text.to_string(),
        from: from.to_string(),
        to: to.to_string(),
        appid: appid.to_string(),
        salt: salt,
        sign: format!("{:x}", sign),
    };

    let client = Client::new();
    let response = client.get(url).query(&query_params).send();
    match response {
        Ok(res) => {
            let ret: TranslateResults = res.json().unwrap();
            println!("request result: {:?}", ret);
        }
        Err(e) => {
            println!("request error: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::translate;

    #[tokio::test]
    async fn test_translate() {
        translate("hello", "auto", "zh").await;
    }
}
