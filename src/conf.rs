use crate::util::*;
use serde::{Deserialize, Serialize};
use serde_yaml;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Conf {
    pub kind: String,
    pub local: LocalConf,
    pub remote: RemoteConf,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalConf {
    pub dir: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteConf {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

impl Conf {
    pub fn _defaults() -> Self {
        Self {
            kind: String::from("s3d"),
            local: LocalConf {
                dir: String::from("."),
            },
            remote: RemoteConf {
                endpoint: String::from("localhost:9000"),
                access_key: String::from(""),
                secret_key: String::from(""),
            },
        }
    }

    pub async fn load(file_path: &str) -> ResultOrAnyErr<Conf> {
        let mut contents = String::new();
        let mut file = File::open(file_path).await?;
        file.read_to_string(&mut contents).await?;
        let conf: Conf = serde_yaml::from_str(&contents)?;
        Ok(conf)
    }
}
