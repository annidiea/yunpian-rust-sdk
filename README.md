
#云片短信SDK

官网5年前提交的sdk先已不能使用，特更新了一个



This crate offers:

*   官网5年前提交的sdk已不能使用，特更新了一个

Features:

*   macOS, Windows and Linux support;


### Installation

将所需版本的crate放入dependencies的部分 `Cargo.toml`:

```toml
[dependencies]
yunpian-sdk = "*"
```

## Example

```rust

#[tokio::main]
async fn main() {
    let api_key = "云片api_key";
    let sms = yunpian_sdk::Yunpian::new(api_key);
    let text = "【安妮蝶网络】您的验证码为1024，10分钟内有效，请勿泄露。如非本人操作，请忽略本短信。";
    let phone = "18610996705";
    match sms.single_send(text, phone).await{
        Ok(b) => {
            if b == true {
                println!("发送成功")
            }else{
                print!("发送失败")
            }
        },
        Err(e) => {
            println!("err: {}", e)
        }
    };
}


```

### Crate Features

* crate's features:


### API Documentation

Please refer to the [crate docs](https://docs.rs/yunpian-sdk)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
