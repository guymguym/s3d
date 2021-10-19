use crate::util::*;
use serde::{Deserialize, Serialize};
use serde_yaml;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Conf {
    pub kind: String,
    pub hub: Option<HubConf>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HubConf {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

impl Conf {
    pub fn defaults() -> Self {
        Self {
            kind: "s3d".to_string(),
            hub: None,
        }
    }

    pub async fn load(file_path: &str, ignore_not_found: bool) -> ResultOrAnyErr<Conf> {
        let mut contents = String::new();
        let mut file = match File::open(file_path).await {
            Ok(file) => file,
            Err(err) => {
                if ignore_not_found {
                    return Ok(Conf::defaults());
                    // return Err(err);
                }
                println!("{:?}", err);
                return Ok(Conf::defaults());
                // return Err(err);
            }
        };
        file.read_to_string(&mut contents).await?;
        let conf: Conf = serde_yaml::from_str(&contents)?;
        Ok(conf)
    }
}
