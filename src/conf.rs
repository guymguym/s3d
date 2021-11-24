use serde::{Deserialize, Serialize};
use serde_yaml;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Conf {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub local: LocalConf,
    pub remotes: Vec<RemoteConf>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalConf {
    pub port: u16,
    pub ttl: String,
    pub max_disk_usage: String,
    pub fuse_mount_point: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteConf {
    pub name: String,
    pub endpoint: String,
    pub profile: Option<String>,
}

impl Conf {
    pub async fn load(dir: &str) -> anyhow::Result<Conf> {
        let conf_path = Path::new(dir).join("config");
        let contents = tokio::fs::read_to_string(conf_path).await?;
        let conf: Conf = serde_yaml::from_str(&contents)?;
        Ok(conf)
    }

    // pub fn _defaults() -> Self {
    //     Self {
    //         schema: String::from("https://s3d.rs/schemas/v0.0.1/config.schema.json"),
    //         local: LocalConf {
    //             ttl: 0,
    //         },
    //         remote: RemoteConf {
    //             endpoint: String::from("localhost:9000"),
    //             access_key: String::from(""),
    //             secret_key: String::from(""),
    //         },
    //     }
    // }
}
