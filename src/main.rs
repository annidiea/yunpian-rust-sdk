
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
