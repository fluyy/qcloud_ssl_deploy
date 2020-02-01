
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug)]
#[allow(non_snake_case)] 
pub struct WssClient <'a>{
    conf : &'a super::client::QcloudConf,
    end_point : String,
    service  : String,
}

pub fn new_client(conf : &super::client::QcloudConf) -> WssClient {
    WssClient {
        conf: conf,
        end_point: "wss.tencentcloudapi.com".to_owned(),
        service : "wss".to_owned(),
    }
}

#[allow(non_snake_case)] 
#[derive(Debug, Serialize, Deserialize)]
pub struct  DescribeCertListReq {
    pub Offset: u32,
    pub Limit: u32,
    pub CertType: String,
    pub WithCert: String,
    pub ModuleType: String, 
    pub AltDomain: String,
    pub Id: String,
}

#[allow(non_snake_case)] 
#[derive(Debug, Deserialize)]
pub struct SSLCertificate {
    pub Id : String,
    pub CertType : String,
    pub CertBeginTime: String,
    pub CertEndTime : String,
}

#[allow(non_snake_case)] 
#[derive(Debug, Deserialize)]
pub struct DescribeCertListRsp {
    pub Error: Option<super::client::QCloudError>,
    pub CertificateSet: Vec<SSLCertificate>,
    pub TotalCount: Option<u32>,
    pub RequestId: String,
}

type QcloudDescribeCertListRsp = super::client::QCloudResponse<DescribeCertListRsp>;

impl super::client::QcloudCommParams for DescribeCertListReq {
    fn action(&self) -> &str{
        "DescribeCertList"
    }
    fn version(&self) -> &str {
        "2018-04-26"
    }
    fn region(&self) -> Option<&str> {
        None
    }
}

#[allow(non_snake_case)] 
#[derive(Debug, Serialize, Deserialize)]
pub struct  UploadCertReq {
    pub CertType: String,
    pub Cert: String,
    pub ProjectId : String,
    pub Key : String,
    pub Alias: String,
    pub ModuleType: String, 
}

#[allow(non_snake_case)] 
#[derive(Debug, Deserialize)]
pub struct UploadCertRsp {
    pub Error: Option<super::client::QCloudError>,
    pub RequestId: String,
    pub Id: String,
}
type QcloudUploadCertRsp = super::client::QCloudResponse<UploadCertRsp>;

impl super::client::QcloudCommParams for UploadCertReq {
    fn action(&self) -> &str{
        "UploadCert"
    }

    fn version(&self) -> &str {
        "2018-04-26"
    }
    fn region(&self) -> Option<&str> {
        None
    }
}

#[allow(non_snake_case)] 
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCertReq {
    pub Id: String,
    pub ModuleType: String, 
}

#[allow(non_snake_case)] 
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCertRsp {
    RequestId: String,
}

impl super::client::QcloudCommParams for DeleteCertReq {
    fn action(&self) -> &str{
        "DeleteCert"
    }

    fn version(&self) -> &str {
        "2018-04-26"
    }
    fn region(&self) -> Option<&str> {
        None
    }
}

// {\"Response\":{\"Error\":{\"Code\":\"InvalidParameter\",\"Message\":\"The value type of parameter `AltDomain` is not valid.\"},\"RequestId\":\"9f090251-59c8-4e61-b431-b30b6a65f080\"}}"

impl <'a> WssClient <'a>{
    #[allow(non_snake_case)] 
    pub async fn UploadCert(&mut self, req : &UploadCertReq) -> Result<QcloudUploadCertRsp, super::error::Error> {
        let resp_text = super::client::request(&self.conf, self.end_point.as_str(), self.service.as_str(), req).await?;
        println!("response text: {:?}", resp_text);
        let resp_result : QcloudUploadCertRsp = serde_json::from_str(&resp_text)?;
        Ok(resp_result)
    }

    #[allow(non_snake_case)] 
    pub async fn DescribeCertList(&mut self, req : &DescribeCertListReq) -> Result<QcloudDescribeCertListRsp, super::error::Error> {
        let resp_text = super::client::request(&self.conf, self.end_point.as_str(), self.service.as_str(), req).await?;
        println!("response text: {:?}", resp_text);
        let resp_result : QcloudDescribeCertListRsp = serde_json::from_str(&resp_text)?;
        Ok(resp_result)
    }

    #[allow(non_snake_case)] 
    pub async fn DeleteCert(&mut self, req : &DeleteCertReq) -> Result<DeleteCertRsp, super::error::Error> {
        let resp_text = super::client::request(&self.conf, self.end_point.as_str(), self.service.as_str(), req).await?;
        println!("response text: {:?}", resp_text);
        let resp_result : DeleteCertRsp = serde_json::from_str(&resp_text)?;
        Ok(resp_result)
    }
}

