use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use serde::{Serialize, Deserialize};
use reqwest;

type HmacSha256 = Hmac<Sha256>;

#[allow(non_snake_case)] 
#[derive(Debug, Serialize, Deserialize)]
pub struct QCloudError {
    pub Code : String,
    pub Message : String,
}

#[allow(non_snake_case)] 
#[derive(Debug, Serialize, Deserialize)]
pub struct QCloudResponse <T>{
    pub Response : T ,
}

#[allow(non_snake_case)] 
#[derive(Debug,Serialize, Deserialize)]
pub struct QcloudConf{
    pub secret_id: String,
    pub secret_key: String,
}

pub trait QcloudCommParams {
    fn action(&self) -> &str;
    fn version(&self) -> &str;
    fn region(&self) -> Option<&str>;
}

fn hmac_sha256(key : &Vec<u8>, data : &str) -> Vec<u8> {
    let mut mac = HmacSha256::new_varkey(key).expect("init hmac256");
    mac.input(data.as_bytes());
    let hex_result = mac.result().code().to_vec();
    hex_result
}

fn gen_authorization_v3(conf : &QcloudConf, request : &reqwest::Request, service : &str) ->String {
    let algorithm = "TC3-HMAC-SHA256";
    let mut header_data = String::with_capacity(64);

    let content_type = request.headers().get(reqwest::header::CONTENT_TYPE);
    match content_type {
        Some(type_value) => header_data.push_str(format!("content-type:{}\n", type_value.to_str().unwrap()).as_str()),
        None => println!("content type is none"),
    }
    
    let host = request.url().host_str();
    match host {
        Some(host_data) => header_data.push_str(format!("host:{}\n", host_data).as_str()),
        None => { println!("no host found");}
    }

    let mut hasher = Sha256::new();
    match request.body() {
        Some(body) => {
            hasher.input(body.as_bytes().unwrap());
            println!("request body:{:?}", std::str::from_utf8(body.as_bytes().unwrap()));
        },
        _ => {}
    }
    let payload_hash = hex::encode(hasher.result().as_slice());  

    // 1. 拼接规范请求串
    let method = request.method().as_str();
    let uri = "/";
    let query_string = request.url().query().unwrap_or("");
    let signed_headers = "content-type;host";
    let canonical_request = format!("{}\n{}\n{}\n{}\n{}\n{}", 
        method, uri, query_string, header_data, signed_headers, payload_hash);
    println!("canonical_request:{}", canonical_request);

    // 2. 拼接待签名字符串
    let mut hasher = Sha256::new();
    hasher.input(canonical_request.as_str());
    let canonical_request_hash = hex::encode(hasher.result().as_slice());
    let utc_time: DateTime<Utc> = Utc::now();
    let timestamp = utc_time.timestamp();
    let today = format!("{}", utc_time.format("%Y-%m-%d"));
    let scope = format!("{}/{}/tc3_request", today, service);
    let string_to_sign = format!("{}\n{}\n{}\n{}",algorithm, timestamp, scope, canonical_request_hash);
    println!("string_to_sign:{}", string_to_sign);

    // 3. 计算签名
    let initial_key = "TC3".to_string() + conf.secret_key.as_str();
    let secret_date = hmac_sha256(&initial_key.as_bytes().to_vec(), &today);
    let secret_service = hmac_sha256(&secret_date, service);
    let secret_signing = hmac_sha256(&secret_service, "tc3_request");
    let signature = hmac_sha256(&secret_signing, string_to_sign.as_str());

    // 4. 拼接 Authorization
    let hashed_signature = hex::encode(signature);
    let authorization = format!("{} Credential={}/{}, SignedHeaders={}, Signature={}",
        algorithm, conf.secret_id, scope,signed_headers,hashed_signature);
    println!("authorization:{}", authorization);
    authorization 
}

pub async fn request<T>(conf : &QcloudConf, host : &str, service : &str, params : &T) -> Result<String, reqwest::Error> 
where
    T: QcloudCommParams+serde::Serialize
{
    let mut api_host = host.trim().to_string();
    if !host.starts_with("https://") {
        api_host = format!("https://{}", api_host);
    }
    let utc_time: DateTime<Utc> = Utc::now();

    let cli = reqwest::Client::new();
    let mut request_builder = cli.post(api_host.as_str()).json(params).
        header("X-TC-Action", params.action()).
        header("X-TC-Version", params.version()).
        header("X-TC-Timestamp", utc_time.timestamp());

    match params.region() {
        Some(region) => {
            request_builder = request_builder.header("X-TC-Region", region);
        },
        None => {},
    }

    let mut request = request_builder.build()?;
    println!("{:#?}",request);
    let authorization = gen_authorization_v3(conf, &request, service);
    request.headers_mut().insert("Authorization", authorization.parse().unwrap());  
    let response = cli.execute(request).await?.text().await?;
    Ok(response)
}





