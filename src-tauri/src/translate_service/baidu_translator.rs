use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_plugin_http::reqwest;

use crate::{errors::RosettaError, StoreState};

use super::TranslateResults;

const KEY: &str = "BAIDU";

#[derive(Serialize)]
struct QueryParams {
    q: String,
    from: String,
    to: String,
    appid: String,
    salt: String,
    sign: String,
}

#[derive(Deserialize)]
struct TranslatorConfig {
    appid: String,
    secret: String,
}

pub async fn translate(
    app_handle: tauri::AppHandle,
    text: &str,
    from: &str,
    to: &str,
) -> Result<TranslateResults, RosettaError> {
    let state = app_handle.state::<StoreState>();
    let store = state.0.clone();
    let baidu_config = store.get("baidu").ok_or(RosettaError::ConfigError(
        "Config for baidu translator is empty".to_string(),
    ))?;
    let translator_config: TranslatorConfig =
        serde_json::from_value(baidu_config).map_err(|e| {
            RosettaError::ConfigError(format!(
                "Config for baidu translator deserialize error: {}",
                e
            ))
        })?;
    let appid = translator_config.appid;
    let secret = translator_config.secret;

    let url = "https://fanyi-api.baidu.com/api/trans/vip/translate";

    let salt = nanoid!();

    let str = format!("{}{}{}{}", appid, text, salt, secret);
    let sign = md5::compute(str);

    let query_params = QueryParams {
        q: text.to_string(),
        from: from.to_string(),
        to: to.to_string(),
        appid: appid.to_string(),
        salt,
        sign: format!("{:x}", sign),
    };
    // 使用reqwest，携带query_params发起异步请求
    let client = reqwest::Client::new();
    let response = client.get(url).query(&query_params).send().await?;
    let ret: TranslateResults = response.json().await?;
    println!("request result: {:?}", ret);
    Ok(ret)

    // match response {
    //     Ok(res) => {
    //         let ret: TranslateResults = res.json().await.unwrap();
    //         println!("request result: {:?}", ret);
    //         Ok(ret)
    //     }
    //     Err(e) => {
    //         println!("request error: {}", e);
    //         Err(e)
    //     }
    // }
}
