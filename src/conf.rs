use std::path::PathBuf;

use crate::util::*;
use serde::{Deserialize, Serialize};
use serde_yaml;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Conf {
    pub s3d: String,
    pub local: LocalConf,
    pub remotes: Vec<RemoteConf>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalConf {
    pub ttl_seconds: String,
    pub max_disk_usage: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteConf {
    pub name: String,
    pub endpoint: String,
    pub profile: String,
}

impl Conf {
    pub async fn load(file_path: &PathBuf) -> ResultOrAnyErr<Conf> {
        let mut contents = String::new();
        let mut file = File::open(file_path).await?;
        file.read_to_string(&mut contents).await?;
        let conf: Conf = serde_yaml::from_str(&contents)?;
        Ok(conf)
    }

    // pub fn _defaults() -> Self {
    //     Self {
    //         s3d: String::from("config"),
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
