mod client;
mod wss;
mod cdn;
mod error;
use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::Read;
use chrono::{DateTime, Local};
use chrono::offset::TimeZone;
use serde::{Serialize, Deserialize};

#[allow(unused_imports)] 
use tokio::prelude::*;

struct Argv {
    config : String,
    action : String,
}

fn init_argv() -> Argv {
    let matches = App::new("cert sync tools")
        .version("1.0")
        .author("fluyy")
        .about("sync cert to qcloud")
        .arg(
            Arg::with_name("action")
                .short("a")
                .long("action")
                .help("action value delete, deploy, show")
                .takes_value(true),            
        ).arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("config file path")
                .takes_value(true),
        ).get_matches();
    
    Argv {
        config : matches.value_of("config").unwrap().to_owned(),
        action : matches.value_of("action").unwrap().to_owned(),
    }
}

#[derive(Debug,Serialize, Deserialize)]
struct DomainConf {
    cert_file : String,
    cert_private : String,
    domain_list  : Vec<String>,
}

#[derive(Debug,Serialize, Deserialize)]
struct Config {
    qcloud_conf : client::QcloudConf,
    domain_list : Vec<DomainConf>,
}

fn parse_conf(filename: &String) -> Result<Config, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}


async fn auto_deploy_cloud_cert(conf: &client::QcloudConf, domain_conf : &DomainConf) {
    let mut wss_cli = wss::new_client(conf);
    let mut cert_file = File::open(&domain_conf.cert_file).unwrap();
    let mut cert_content = String::new();
    cert_file.read_to_string(&mut cert_content).unwrap();

    let mut key_file = File::open(&domain_conf.cert_private).unwrap();
    let mut key_content = String::new();
    key_file.read_to_string(&mut key_content).unwrap();

    let req = wss::UploadCertReq {
        Alias : "auto_upload".to_owned(),
        Cert : cert_content.to_owned(),
        CertType : "SVR".to_owned(),
        ModuleType : "ssl".to_owned(),
        Key : key_content.to_owned(),
        ProjectId : "0".to_owned(),
    };
    match wss_cli.UploadCert(&req).await {
        Ok(result) =>  {
            let response : wss::UploadCertRsp = result.Response;
            let mut cdn_cli = cdn::new_client(conf);

            for domain in domain_conf.domain_list.iter() {
                let update_result = cdn_cli.UpdateDomainConfig(&cdn::UpdateDomainConfigReq{
                    Domain : domain.to_owned(),
                    Https: cdn::Https{
                        Switch   : "on".to_owned(), // on,off
                        Http2    : "on".to_owned(), // on,off
                        CertInfo : cdn::ServerCert{
                            CertId: response.Id.to_owned(),
                        },
                    },     
                }).await;
                match update_result {
                    Ok(resp) => {
                        if resp.Response.Error.is_some() {
                            println!("Error: domain:{},cert:{},reqid:{},err:{:?}", domain, response.Id, resp.Response.RequestId, resp.Response.Error);
                        } else {
                            println!("Success: domain:{},cert:{},reqid:{} ", domain, response.Id, resp.Response.RequestId);
                        }
                    },
                    Err(error) => {
                        println!("Error: update domain:{} failed,err:{}", domain, error);
                    },
                }
            }
        },
        Err(err) => {
            println!("Error: UploadCert {:?}",  err);
        }
    }
}


async fn auto_delete_cloud_cert(conf: &client::QcloudConf) {

    let mut wss_cli = wss::new_client(conf);
    let mut cloud_cert_list : Vec<wss::SSLCertificate> = Vec::with_capacity(0);
    let req = wss::DescribeCertListReq {
        Offset: 1,
        Limit: 20,
        CertType: "SVR".to_owned(),
        ModuleType: "ssl".to_owned(),
        WithCert: "0".to_owned(),
        AltDomain: "".to_owned(),
        Id: "".to_owned(),
    };
    match wss_cli.DescribeCertList(&req).await {
        Ok(result) => {
            println!("DescribeCertList: {:?}", result);
            let mut response : wss::DescribeCertListRsp = result.Response;
            if response.Error.is_some() {
                println!("ReqId:{},DescribeCertList failed : {:?}", response.RequestId, response.Error.unwrap());
                return 
            } else {
                cloud_cert_list.append(&mut response.CertificateSet);
            }
        }, 
        Err(err) => {
            println!("DescribeCertList Error: {:?}",  err);
        }
    }
    let now : DateTime<Local>= Local::now();
    for cert in cloud_cert_list.iter() {
        match Local.datetime_from_str(&cert.CertEndTime, "%Y-%m-%d %H:%M:%S") {
            Ok(endtime) => {     
                if endtime.signed_duration_since(now).num_seconds() > 0 {
                    println!("ok: ssl cert id: {}, start:{}, end:{}", cert.Id, cert.CertBeginTime, cert.CertEndTime);
                } else {
                    let del_result = wss_cli.DeleteCert(&wss::DeleteCertReq{ Id : cert.Id.to_owned(), ModuleType: "ssl".to_owned(), }).await;
                    match del_result {
                        Ok(del_result) => {
                            println!("invalid: ssl cert id: {}, start:{}, end:{}, resp:{:?}", cert.Id, cert.CertBeginTime, cert.CertEndTime, del_result);
                        },
                        Err(err) => {
                            println!("invalid: ssl cert id: {}, start:{}, end:{}, err:{:?}", cert.Id, cert.CertBeginTime, cert.CertEndTime, err);
                        },
                    }
                }
            },
            Err(err) => {
                println!("ssl cert id: {}, start:{}, end:{} parse endtime failed:{:?}", cert.Id, cert.CertBeginTime, cert.CertEndTime, err);
            }
        }
    }
}


#[tokio::main]
async fn main() {
    let argv = init_argv();
    let conf = parse_conf(&argv.config).expect("parse config file failed");
    println!("config:{:?}", conf);
    if argv.action == "delete" {
        auto_delete_cloud_cert(&conf.qcloud_conf).await;
    } else if argv.action == "deploy" {
        for domain_conf in conf.domain_list.iter(){
            auto_deploy_cloud_cert(&conf.qcloud_conf, domain_conf).await;
        }
    }
}
