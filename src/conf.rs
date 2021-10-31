use serde::{Deserialize, Serialize};
use serde_yaml;
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Conf {
    pub kind: String,
    pub local: LocalConf,
    pub remotes: Vec<RemoteConf>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalConf {
    pub port: u16,
    pub ttl: String,
    pub max_disk_usage: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteConf {
    pub name: String,
    pub endpoint: String,
    // pub profile: String,
}

impl Conf {
    pub async fn load(file_path: &PathBuf) -> anyhow::Result<Conf> {
        let mut contents = String::new();
        let mut file = File::open(file_path).await?;
        file.read_to_string(&mut contents).await?;
        let conf: Conf = serde_yaml::from_str(&contents)?;
        Ok(conf)
    }

    // pub fn _defaults() -> Self {
    //     Self {
    //         kind: String::from("s3d.rs/config"),
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
