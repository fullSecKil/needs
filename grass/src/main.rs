use std::thread::sleep;
use std::time::Duration;

use rand::distributions::{Alphanumeric, DistString};
use reqwest::{Client, Error};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..1 {
        generate_email_and_register_accent().await?;
    }

    Ok(())
}


async fn generate_email_and_register_accent() -> Result<(), Error> {
    let generate_email_url = "https://22.do/zh/mailbox/generate";
    let grass_url = "https://api.getgrass.io/register";
    let email_response = send_get_request(generate_email_url).await?;

    println!("email body : {}", email_response);

    let json: Value = serde_json::from_str(&email_response).unwrap();
    let value = &json["data"]["address"]["email"];

    let mut body = json!({
    "email": "",
    "password": "!QAZ2wsx",
    "role": "USER",
    "referralCode": "Fs6cBbg5xRRxHFj",
    "username": "3241ff22",
    "recaptchaToken": "03AFcWeA7EHzk9WG07kC3njkXMe7Xqw3_J3Ajh8F7qdU9kLtlxHdRdqvCeaYQ8slACOJ4ZjwS15rsEMrKuNgIUM0bWCxbirXW70CUFfWpwL894GFi5aa69HiV8sodSnkRfM-7pwW6N1UN69yFvWb4FrwZLdNqjgxBisItoMvHwfnlvXpe62lGUnLy_m0GVH-Vy3ZWzryG_89aAznvcMQvNRN5GGH5Iib7MRPJVgW7oeLoqxtdJ0ksjKMPJheZi5kgUObb5Jzhr0C0gy4--_Y0BDXejZaPJNm9osRQlPoigKrEqZtJ1haXf9p3-oI-sZnch_GrPSWGpbJQvb-jKdn-6prLwqBsIRwRbT4Xzi8Y8WrCz2W3OcSAllmh0R3xTmltMrFZAAyuXdGTk0crwyxnlm5PT9usUjFhfCDMAXO74UuOYb2v6NpTish0Uz5MxNwKEerxaLFabu0nGzgLpKdnHZJk3-TL0L40QBdCcsVdwRju1GB6p0Vb5aHQNSoYuf237dVf8Mrx_9gV_FvjXQ2WhrX_wpQAdCmMq56TIReXn3Ue1I3pUCLQQvg1D03V2XfcuGWEac5fwSiRmXj9bfnI2AXaHAvSFk2LkM7RNxCKLF8jviGMHOis8-QgokSVey5wAa8ZSIAoZrpqAJ9Y7JAcVCeBOU0IEYD1QdqGC1Srwq95J8clAyaXQx_kf1xRfiLCujCdaR1-TqXbJosXsXoco9nUeIF__NA1-JuNxIEOstSM7Lhu6Qf5Jn8xwzEnMm7Wn3iyxsr6Ml5LOOe1ShpBrOSKRYmQaA9QcwDZaGuMkwvHBBbP--F-X3TaZZaD5Oa1b2uvFzSLJs0A9o76i8p1mBDnRpcWIcnle-7uYu3KtGRA65-xBDvrFILSB85sIJgUsxNrcZJbyaozrnU9xW9gy9BeBd8Dm8SuWpKMsrsAs-SZsmOj6lBoFziKS0vJ5SdJCzbVe0ZCt1X_qYkwc-6Da1QyziRGS7shPucVQdF2SVz1KbCZurTveuU_QAM8-Z9QEBpcr7OTt-Kd3D5i5CF3vudS7SmbiYw9nwG4H9heUr__DEmLVaygiNV8L5XxcAlx5eJMe-g0vI6EihB3RXemlCrozooBo_PR-LDdIRlucu6Le0GJ2D-B1wxq5DIawcy3rhHzHAHBmbhGi2r4m5FoSWKXKJmu0J3fwlzhgUYNBgKQXFehcCxw4zhIsENNgMzJlKgRNonETaOJ_Zm0nGQm1EL80_v8rYiGVyvKoRPEiaQB8CLgleeXa4XrU25odN0gSNNWOQoPRfKdg5gwSiMaLorNghPd68fzNtxNUhGW8lJfZo01JeN9DO-cTxwK5TWiqk6LAgwpfViNUV7NnHK6aLxTUj37V2C1t2-LARl6sTNLN3k03v-ufqWFu9tKYOvGYhSvf9swYAKFFkiN3-ZQGOQGjw1jXvFtLPHz1MmVvxWTuU5LZNpzLu4pstYEo_JKl-c8KPnYuPHjgjEevydSMEXsQMtMY2FUMrKbHRhUhCHNLH-2YXYqMShSpNOWDXW4g498JYAkG47u0q9ONT_Yud6Cyt9omsKkNgTC419ATlt7-c-QdKaF2PCP5MPsQxTuVrfrSDL3yIFQTArbIJY3wU7O26MPn71bvzdjPWqJisGg1zDWobaNut7qp1g8-xQMgHj3q2S91IgtzTDIpjHA1RzNdUga4bGdJ4NJYfdYCZx_pIhA9WGcOXNx2aEatzTPwvPLsGS9H0GvLOqM0tP1hbkSl5FmVgkHkpLvwyI1-LpJYgxIvy0jd2CfRESuGBxGDWlFYu-ueTUQChXMtH611AiJqvhj3rlfpBcQJIpJuu2P9Shs5ZF6cj9pZzAI2CaQKXcdRhVUsd5IMAnAmAHdge_b-MDAP7Re5Gz1LUO3jRKP1R7gSKNk_0n11BqU-VTAnwg81YZi9046jePedX2n_93jKU-rjj3BrIyxOP77e9J-KbCD5DLElH6Y",
    "listIds": [
        15
    ]});

    body["email"] = json!(value);
    let name = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    body["username"] = json!(name);

    let result = send_post_request(grass_url, &body).await;

    match result {
        Ok(r) => println!("result info : {}", r),
        Err(err) => eprintln!("Error result file: {}", err),
    };

    Ok(())
}


// 发送GET请求
async fn send_get_request(url: &str) -> Result<String, Error> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}

// 发送POST请求
async fn send_post_request(url: &str, json_data: &serde_json::Value) -> Result<String, Error> {
    let client = Client::new();
    println!("post request body : {}", json_data);
    let response = client.post(url).json(json_data).send().await?;
    let body = response.text().await?;
    Ok(body)
}