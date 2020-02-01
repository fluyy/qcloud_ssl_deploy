## qcloud_cert_deploy

腾讯云SSL证书自动化部署工具

由于 ``` let's encrypt ```证书的有效期只有3个月，每次到期都需要手动的更新证书并部署。所以利用
腾讯云API，用rust写了一个小工具用来自动更新证书


### 快速使用

__自动上传证书并部署在对应域名上__

```
qcli -c ./config/config.toml -a deploy 
```

__自动删除过期的证书__

```
qcli -c ./config/config.toml -a delete
```

### 代码目录结构如下：

```
src
├── cdn
│   └── mod.rs   // cdn相关api，目前只封装了
├── client
│   └── mod.rs   // 腾讯云API V3签名，调用相关接口
├── error
│   └── mod.rs   // 错误定义
├── lib.rs
├── main.rs        
└── wss
    └── mod.rs   // ssl证书相关接口（已实现查看，删除，上传三个接口）
```