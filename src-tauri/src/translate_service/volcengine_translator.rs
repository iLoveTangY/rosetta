/**
 * NOTE: 暂时无法使用，没有跑通
 */
use chrono::{Local, Utc};
use hex::encode;
use hmac::{digest::generic_array::GenericArray, Hmac, Mac};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use serde_json::{json, to_string};
use sha2::{Digest, Sha256};

pub fn translate(text: &str, to: &str) {
    let access_key_id = "";
    let secret_access_key = "";

    let body = json!({
        "TargetLanguage": to,
        "TextList": [text],
    });
    let body_str = to_string(&body).unwrap();
    println!("body_str = {}", body_str);
    let body_hash = Sha256::digest(&body_str);
    let body_hash = format!("{:x}", body_hash);

    println!("body_hash = {}", body_hash);

    let now = Utc::now();
    let iso_str = now.format("%Y%m%dT%H%M%SZ").to_string();
    // let iso_str = "20241231T122824Z";
    println!("rfc3399 format = {}", iso_str);

    // CanonicalRequest = HTTPRequestMethod + '\n' + CanonicalURI + '\n' + CanonicalQueryString + '\n' + CanonicalHeaders + '\n' + SignedHeaders + '\n' + HexEncode(Hash(RequestPayload))
    let host = "open.volcengineapi.com";
    let signed_headers = json!({
        // 请求头要按字母排序
        "content-type": "application/json",
        "host": host,
        "x-content-sha256": body_hash,
        "x-date": iso_str,
    });

    let mut signed_str = String::new();
    let mut md_signed_headers = String::new();

    let signed_headers_keys = signed_headers.as_object().unwrap().keys();
    for key in signed_headers_keys {
        let v = signed_headers.get(key).unwrap().as_str().unwrap();
        signed_str.push_str(&format!("{}:{}\n", key, v));
        md_signed_headers.push_str(&format!("{};", key));
    }
    // 去掉最后一个分号
    md_signed_headers.pop();
    println!("signed_str = {}", signed_str);
    println!("signed headers = {}", md_signed_headers);

    let method = "POST";
    let norm_uri = "/";
    let query_string = "Action=TranslateText&Version=2020-06-01";
    let canoncial_request = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        method, norm_uri, query_string, signed_str, md_signed_headers, body_hash
    );
    println!("canoncial_request = {}", canoncial_request);
    let hashed_canon_req = format!("{:x}", Sha256::digest(canoncial_request));
    println!("hased_canon_req: {}", hashed_canon_req);

    let region = "cn-north-1";
    let service = "translate";
    let request = "request";
    let k_date = hmac_sha256(&iso_str[0..8], secret_access_key.as_bytes());
    let k_region = hmac_sha256(region, &k_date);
    let k_service = hmac_sha256(service, &k_region);
    let signing_key = hmac_sha256(request, &k_service);

    println!("signing_key = {}", encode(signing_key.clone()));
    let algorithm = "HMAC-SHA256";
    let credential_scope = format!("{}/{}/{}/request", &iso_str[0..8], region, service);
    let signing_str = format!(
        "{}\n{}\n{}\n{}",
        algorithm, iso_str, credential_scope, hashed_canon_req
    );
    println!("signing_str = {}", signing_str);
    let sign = hmac_sha256(&signing_str, &signing_key);
    let authorization = format!(
        "{} Credential={}/{}, SignedHeaders={}, Signature={}",
        algorithm,
        access_key_id,
        credential_scope,
        md_signed_headers,
        encode(sign)
    );

    // let mut headers = HeaderMap::with_capacity(4);
    // headers.insert(
    //     "Authorization",
    //     HeaderValue::from_str(&authorization).unwrap(),
    // );
    // headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
    // headers.insert("Host", HeaderValue::from_str(host).unwrap());
    // headers.insert(
    //     "X-Content-Sha256",
    //     HeaderValue::from_str(&body_hash).unwrap(),
    
    // );  
    // headers.insert("X-Date", HeaderValue::from_str(&iso_str).unwrap());

    let mut headers = HeaderMap::with_capacity(5);
    headers.insert(
        "Authorization",
        HeaderValue::from_str("HMAC-SHA256 Credential=AKLTYTUxYzQ4YjRmOWRiNGJlOWFiMzZhNjhmMzgxMTlkZWU/20241231/cn-north-1/translate/request, SignedHeaders=content-type;host;x-content-sha256;x-date, Signature=7d1c5daad9f353eadd09cf0ea2f78de3ebc9630719952efc3a1f908a103b9f09").unwrap(),
    );
    headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
    headers.insert("Host", HeaderValue::from_str("open.volcengineapi.com").unwrap());
    headers.insert(
        "X-Content-Sha256",
        HeaderValue::from_str("5b531d1f4a82214c5a0af857be50e3c162e19dbc2ba1ef04a591f5432ed3dc15").unwrap(),
    );
    headers.insert("X-Date", HeaderValue::from_str("20241231T132826Z").unwrap());
    
    println!("Headers: {:?}", headers);

    let url = format!("https://{}/?Action=TranslateText&Version=2020-06-01", host);
    let url = "https://open.volcengineapi.com/?Action=TranslateText&Version=2020-06-01";

    println!("url = {}", url);
    let client = Client::new();
    let body = json!({"type":"Text", "payload":"{\"TargetLanguage\":\"zh\",\"TextList\":[\"hello\"]}"});
    println!("body = {}", body.to_string());
    let response = client
        .post(url)
        .headers(headers)
        .body(body.to_string())
        .send();
    match response {
        Ok(res) => {
            println!("request result: {:?}", res.text())
        }
        Err(e) => println!("request error: {}", e),
    }
}

fn hmac_sha256(message: &str, key: &[u8]) -> Vec<u8> {
    // println!("message = {}, key = {}", message, key);
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(message.as_bytes());
    mac.finalize().into_bytes().as_slice().to_owned()
    // format!("{:x}", mac.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use super::translate;

    #[test]
    fn test_translate() {
        translate("hello", "zh");
    }
}
