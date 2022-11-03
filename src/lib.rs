

//! This crate offers:
//! # 云片 短信发送sdk
//! Features:
//! *   云片5年前提交的sdk已不能使用，特更新了一个
//! 
//! 
//! ## Installation
//! 将所需版本的crate放入dependencies的部分 `Cargo.toml`:
//! ```toml
//! [dependencies]
//! yunpian-sdk = "*"
//! ```
//! ## Examples
//! ```rust
//! #[tokio::main]
//! async fn main() {
//!     //云片api_key
//!     let api_key = "xxxxx";
//!     let sms = yunpian_sdk::Yunpian::new(api_key);
//!     let text = "【安妮蝶网络】您的验证码为1024，10分钟内有效，请勿泄露。如非本人操作，请忽略本短信。";
//!     let phone = "18610996705";
//!     match sms.single_send(text, phone).await{
//!         Ok(b) => {
//!             if b == true {
//!                 println!("发送成功")
//!             }else{
//!                 print!("发送失败")
//!             }
//!          },
//!         Err(e) => {
//!             println!("err: {}", e)
//!         }
//!      };
//! }
//! ```
//! 
/// 
///
/// 
// --snip--

use serde::{Deserialize, Serialize};
use reqwest::Client;

use std::collections::HashMap;
use std::time::Duration;
use std::io::{Result, Error, ErrorKind};


const URL: &str = "https://sms.yunpian.com/v2/sms/single_send.json";
const SUCC_CODE: i64 = 0i64;

#[derive(Debug, Deserialize, Serialize)]
struct SingleSendResult {
    code: i64,
    msg: String,
    count: i64,
    fee: f64,
    unit: String,
    mobile: String,
    sid: i64,
}

pub struct Yunpian<'a> {
    api_key: &'a str,
    http_client: Client,
}

impl<'a> Yunpian<'a>{
    pub fn new(api_key: &'a str) -> Yunpian {
        Yunpian{
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn single_send(&self, text: &str, phone: &str) -> Result<bool> {
        let mut param = HashMap::new();
        param.insert("text", text);
        param.insert("mobile", phone);
        param.insert("apikey", &self.api_key);

        let response = match self.http_client
        .post(URL)
        .form(&param)
        .timeout(Duration::from_secs(10))
        .send().await{
            Ok(t) => t,
            Err(e) => {
                let err_str = format!("{}", e);
                return Err(Error::new(ErrorKind::Other, err_str))
            }
        };

        let response_obj: SingleSendResult = match response.json().await{
            Ok(t) => t,
            Err(e) => {
                let err_str = format!("{}", e);
                return Err(Error::new(ErrorKind::Other, err_str))
            }
        };

        if response_obj.code == SUCC_CODE {
            return Ok(true);
        }

        Err(Error::new(ErrorKind::Other, response_obj.msg))

    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_single_send(){
        let api_key = "云片api_key";
        let sms = Yunpian::new(api_key);
        let text = "【天涯网络】您的验证码为1024，10分钟内有效，请勿泄露。如非本人操作，请忽略本短信。";
        let phone = "18610996705";
        let result = match sms.single_send(text, phone).await{
            Ok(b) => b,
            Err(_e) => false
        };

        assert_eq!(true, result)
    }

}