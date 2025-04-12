use nanoid::nanoid;
use serde::Serialize;
use tauri_plugin_http::reqwest;

use crate::errors::RosettaError;

use super::TranslateResults;

#[derive(Serialize)]
struct QueryParams {
    q: String,
    from: String,
    to: String,
    appid: String,
    salt: String,
    sign: String,
}

pub async fn translate(text: &str, from: &str, to: &str) -> Result<TranslateResults, RosettaError> {
    let appid = "20210406000764587";
    let secret = "h73rZUpczILttxsNw5JL";

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

#[cfg(test)]
mod tests {
    use super::translate;

    #[tokio::test]
    async fn test_translate() {
        println!("{:?}", translate("hello", "auto", "zh").await);
    }
}
