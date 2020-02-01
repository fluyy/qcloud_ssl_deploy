use serde::{Serialize, Deserialize};
use serde_json;

#[allow(non_snake_case)] 
#[derive(Debug)]
pub struct CDNClient <'a >{
    conf : & 'a super::client::QcloudConf,
    end_point : String,
    service  : String,
}

pub fn new_client(conf : &super::client::QcloudConf) -> CDNClient {
    CDNClient {
        conf: conf,
        end_point: "cdn.tencentcloudapi.com".to_owned(),
        service : "cdn".to_owned(),
    }
}

#[allow(non_snake_case)] 
#[derive(Debug,Serialize, Deserialize)]
pub struct ServerCert {
    pub CertId : String,
} 

#[allow(non_snake_case)] 
#[derive(Debug, Serialize, Deserialize)]
pub struct Https{
    pub Switch   : String, // on,off
    pub Http2    : String, // on,off
    pub CertInfo : ServerCert,
}

#[allow(non_snake_case)] 
#[derive(Debug, Serialize, Deserialize)]
pub struct  UpdateDomainConfigReq {
    pub Domain : String,
    pub Https: Https,
}

#[allow(non_snake_case)] 
#[derive(Debug, Deserialize)]
pub struct UpdateDomainConfigRsp {
    pub Error: Option<super::client::QCloudError>,
    pub RequestId: String,
}

type QcloudUpdateDomainConfigRsp = super::client::QCloudResponse<UpdateDomainConfigRsp>;

impl super::client::QcloudCommParams for UpdateDomainConfigReq {
    fn action(&self) -> &str{
        "UpdateDomainConfig"
    }
    fn version(&self) -> &str {
        "2018-06-06"
    }
    fn region(&self) -> Option<&str> {
        None
    }
}

impl <'a> CDNClient <'a>{
    #[allow(non_snake_case)] 
    pub async fn UpdateDomainConfig(&mut self, req : &UpdateDomainConfigReq) -> Result<QcloudUpdateDomainConfigRsp, super::error::Error> {
        let resp_text = super::client::request(&self.conf, self.end_point.as_str(), self.service.as_str(), req).await?;
        println!("response text: {:?}", resp_text);
        let resp_result : QcloudUpdateDomainConfigRsp = serde_json::from_str(&resp_text)?;
        Ok(resp_result)
    }
}

