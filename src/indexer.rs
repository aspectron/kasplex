pub mod v1 {

    use crate::api::*;
    use crate::imports::*;
    use crate::result::Result;

    struct Inner {
        url: Url,
    }

    #[derive(Clone)]
    pub struct Indexer {
        inner: Arc<Inner>,
    }

    impl Indexer {
        pub fn try_new(endpoint: Endpoint) -> Result<Self> {
            let url = endpoint.try_to_url()?;

            Ok(Self {
                inner: Arc::new(Inner { url }),
            })
        }

        pub async fn get_indexer_status(&self) -> Result<v1::IndexerStatusResponse> {
            let result =
                get_json::<v1::IndexerStatusResponse>(self.inner.url.join("/info")).await?;
            Ok(result)
        }

        pub async fn get_token_list(&self) -> Result<v1::krc20::TokenListResponse> {
            let result =
                get_json::<v1::krc20::TokenListResponse>(self.inner.url.join("/krc20/tokenlist"))
                    .await?;
            Ok(result)
        }
    }

    #[cfg(test)]
    mod test {

        use super::*;

        #[tokio::test]
        async fn test_get_indexer_status() {
            // let indexer = Indexer::try_new(Endpoint::Url("https://api.kasplex.org/v1".into())).unwrap();
            let indexer = Indexer::try_new(Endpoint::Network {
                network: Network::Testnet11,
                version: 1,
            })
            .unwrap();
            let result = indexer.get_indexer_status().await.unwrap();
            println!("{:?}", result);
        }

        #[tokio::test]
        async fn test_get_token_list() {
            let indexer = Indexer::try_new(Endpoint::Network {
                network: Network::Testnet11,
                version: 1,
            })
            .unwrap();
            let result = indexer.get_token_list().await.unwrap();
            println!("{:?}", result);
        }
    }
}
