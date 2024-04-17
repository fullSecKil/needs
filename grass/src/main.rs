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
    "recaptchaToken": "03AFcWeA6jnigIIYSAjhjI289YrcADYpCX3a52IcxNJhRBLcgq3j8-Xybb1RxTzYZQV_OC8yCosgK1nFAeiGCyVHsZlTNg719C887rq7RCnuVtTPjnkTjzHVXCpPCN2HUtzCyVVjsfdsuBTYz4u9NjhpJrY_PXRQ3HXZ9iNiXCu-Ntl1D3zgr1mOSpsD31kuck93Z8z3giCqA0ayr-_Q3D4ekB7OsUVj5L_6bJFNW8PtRfn13r0guci7abPuTmMbZPeKbLBVDRO3wwG0ZNqD6qIwyvc7UlOfQVzguydH1RSFCp612Y21SdzjEHKyV5qP_9WLhxpdhoUlkJ92fmzQHyq2eMduiD36Hm60rP5aULPSLInKbfy7quNjuHQaHQGqzu5lvYpQutIyhnSCdJDKyuzOoNdocStQzeOwNTu3dgKR4zkOWpj9uvu38ccVBCoxZvSpR4kmeriuwOLbZiiAnSeZsSQzV2y3R1adxIvcVlPmz3k1Mi3GNnmuJ0oh2hjZfkU9R9DYBw-u6JH4YXZ0ivgTf_QDqXlNbOd4SqfrG0JcRp9jJeFq7YZJfxe02_IBfWLU73vlbsmcC5O96O1bFmsfoEzWIHUk5Oza_5KTNT_UxnGq-kgei5XdL9lfqtkhGM4XVJEX2m4DwOuehid8UnrvjaA20ZhMRJ2AnIWMfGTj5dLtgKijlIT0CZh9cAQ3Xhfls91wYIV2b4yOJ0j_-yeWyhrG80CQh84-ZKsXWC0ScFoNyHLfpSIEbS5XkrP9g4WBkuUZY1eu8w-frT9A5_5pbuwEWYYzq3JQMXhEQ6AofW8ug34rlItbgd7BbACRpO1x4j3r1D3xj2p2sfuVPS40Ho0hmHlRLq3mf0w9gFTNoGE_Fj-H9o2eqRjN77Xfh8zW1vVzEuBg5kjF4YIrZJQGtoeGcmhMzHVhREGCF1QICF4c3ldHR6J9SYrY64aixGCUGPcHeic4WbJ_CHiB2olBrrGr6QcOyowOM2SVni_dxFm7017GEAR412CZP64U6ec1yZGX_NFOyZqnyqN1at818yR2l4aqu0ZHHs0i2ishwicLaZuVAR3HhtLHeMFjgGCZfSa6N-eSSkI6TPk7ZoB1WYwSII1bH4oNFxsGjkguKJTR6dI02FLh1bnmNaPxxedUctDl3V5nKT-qnWU33tI3TkuIJOZCLaBduOpZBAFC3DA5WaUV4FQxsOX7R0y1LapBdccWydvPEtACzV-SlLkFZa0z_zklpb6JtixfNxvzwGz3fBdizDBSz7e7T_NcAOPQJlvnPbxvByR4PLb1QIyLTJtnfmUcFJd-XjNCdPJBXjYPyHQaYDxZhFFG21MPiXadkZAPisOone9LHoEnUrqUrixoCOM-9Q4a_yZqnRy2RmEqeB3jBxOCVxZO6COU2D87Kl9g0LVpz5V-odkYQJftkA76mEkb39v1fuN2vI8LvbeYponVLZ7M0poz7pQdbdakMC9etUfjqpSt3D-3vKmaEnf9wvVa1ztgWfbg-92gvpN3yaOD1sKv4fSPp47DczGpwbbj6KsEPymGcHSfgfEp_u72UaNnt-O03NplZCD7gOwctoXFPEnCoaChirouu5sDV1TdmIjRgG57tjYBepgo-sAOd6n2KydC4m6MOPpy6Xgwd8BFCHDgPP4VYbzE-nmomn5fiTBV77GL497rvBhe0OkYfhhjVvGT6c99d309K_NfN2xQH3wVUlaJCe6_3L6CnDpmlnfI5sFVMafvTqPRPJpvS9r3hurvgHXIpVOGA006UDvcc-SiHz3D-QwhVmqNnSb_2INHxtqw_w9MenfwaVbZfDf9G5zMntHVRTZcVqz9-UY-pGMmKiTDReg_cU2XjzPmCVKFMJzbygq4P8ykDOEf2WmPL3Y8x3f3Ddp36rq-56N6bJEJWuGj5SyQwVzXVc5g3Y9Bn_nKm1ks_hxCUjhVJjEhEKsAPcVQGImPTYJGtWKOJwZHF0oKMj5WTBwgSsqyf4xJn4YZbOYqekcrWZ4VuzxI2G6Q",
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
