use crate::{error::Error, result::Result};

pub struct Url(String);

impl From<String> for Url {
    fn from(url: String) -> Self {
        Url(url)
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Url {
    pub fn join(&self, path: &str) -> String {
        format!("{}{}", self.0, path)
    }
}

pub enum Network {
    Mainnet,
    Testnet11,
}

impl Network {
    pub fn try_to_url(&self, version: u32) -> Result<Url> {
        if version != 1 {
            Err(Error::ApiVersionNotSupported(version))
        } else {
            let url = match self {
                Network::Mainnet => format!("https://api.kasplex.org/v{version}"),
                Network::Testnet11 => format!("https://tn11api.kasplex.org/v{version}"),
            };

            Ok(url.into())
        }
    }
}

pub enum Endpoint {
    Url(String),
    Network { version: u32, network: Network },
}

impl Endpoint {
    pub fn try_to_url(self) -> Result<Url> {
        match self {
            Endpoint::Url(url) => Ok(url.into()),
            Endpoint::Network { version, network } => network.try_to_url(version),
        }
    }
}
