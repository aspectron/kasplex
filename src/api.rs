pub mod v1 {

    use crate::imports::*;

    ///
    /// URL path: `//info`
    ///
    /// ```json
    /// {
    ///     "message": "text",
    ///     "result": {
    ///         "daaScore": 77993954,
    ///         "opScore": 779939200003,
    ///         "opTotal": 1200660,
    ///         "tokenTotal": 6502,
    ///         "feeTotal": 56036798346000
    ///     }
    /// }
    /// ```
    ///

    #[derive(Debug, Deserialize)]
    pub struct IndexerStatusResponse {
        pub message: String,
        pub result: IndexerStatusResult,
    }

    #[serde_as]
    #[derive(Debug, Deserialize)]
    pub struct IndexerStatusResult {
        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "daaScore")]
        pub daa_score: u64,

        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "opScore")]
        pub op_score: u64,

        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "opTotal")]
        pub op_total: u64,

        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "tokenTotal")]
        pub token_total: u64,

        // TODO - what is fee total?
        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "feeTotal")]
        pub fee_total: u64,
    }

    // #[derive(Debug, Deserialize)]
    // pub struct IndexerStatusResponse {
    //     pub message : String,
    //     pub result : IndexerStatusResult,
    // }

    // #[derive(Debug, Deserialize)]
    // pub struct IndexerStatusResult {
    //     #[serde(rename = "daaScore")]
    //     pub daa_score: u64,
    //     #[serde(rename = "opScore")]
    //     pub op_score: u64,
    //     #[serde(rename = "opTotal")]
    //     pub op_total : u64,
    //     #[serde(rename = "tokenTotal")]
    //     pub token_total : u64,
    //     #[serde(rename = "feeTotal")]
    //     // TODO - what is fee total?
    //     pub fee_total : u64,
    // }

    pub mod krc20 {
        use crate::imports::*;

        ///
        /// URL path: `/krc20/tokenlist`
        ///
        /// ```json
        /// {
        ///     "message": "text",
        ///     "prev": "text",
        ///     "next": "text",
        ///     "result": {
        ///         "tick": "KASP",
        ///         "max": 2100000000000000,
        ///         "lim": 100000000000,
        ///         "dec": 8,
        ///         "daas": 78000000,
        ///         "daae": 88000000,
        ///         "minted": 1500000000000000,
        ///         "opScoreAdd": 77993954,
        ///         "opScoreMod": 79993666,
        ///         "state": "deployed",
        ///         "hashRev": "eb1482705b07af..",
        ///         "mtsAdd": "1712808987852"
        ///     }
        /// }
        /// ```
        ///

        #[derive(Debug, Deserialize)]
        pub struct TokenListResponse {
            pub message: String,
            pub next: String,
            pub prev: String,
            pub result: Vec<Token>,
        }

        #[serde_as]
        #[derive(Debug, Deserialize)]
        pub struct Token {
            pub tick: String,
            #[serde_as(as = "DisplayFromStr")]
            pub max: u128,

            // TODO - rename to "limit"?
            #[serde_as(as = "DisplayFromStr")]
            #[serde(rename = "lim")]
            pub limit: u128,

            #[serde_as(as = "DisplayFromStr")]
            pub dec: u64,

            // #[serde_as(as = "DisplayFromStr")]
            // #[serde(rename = "daas")]
            // pub mint_start_daa_score : u64,

            // // TODO - rename to ?
            // #[serde_as(as = "DisplayFromStr")]
            // #[serde(rename = "daae")]
            // pub mint_end_daa_score : u64,

            // TODO - rename to total_minted ?
            #[serde_as(as = "DisplayFromStr")]
            pub minted: u128,

            #[serde_as(as = "DisplayFromStr")]
            #[serde(rename = "opScoreAdd")]
            pub op_score_added: u64,

            #[serde_as(as = "DisplayFromStr")]
            #[serde(rename = "opScoreMod")]
            pub op_score_modified: u64,

            // TODO - what are the variants?
            pub state: State,

            #[serde(rename = "hashRev")]
            // TODO - rename to hash_revision ?
            pub hash_rev: Hash,

            #[serde(rename = "mtsAdd")]
            // TODO - what is mts ?  time stamp in milliseconds? should be `ts_msec_added`
            #[serde_as(as = "DisplayFromStr")]
            pub mts_add: u64,
        }

        ///
        /// URL path: `//krc20/token/{tick}`
        ///
        /// ```json
        /// {
        ///     "message": "text",
        ///     "result": {
        ///         "tick": "KASP",
        ///         "max": 2100000000000000,
        ///         "lim": 100000000000,
        ///         "dec": 8,
        ///         "daas": 78000000,
        ///         "daae": 88000000,
        ///         "minted": 1500000000000000,
        ///         "opScoreAdd": 77993954,
        ///         "opScoreMod": 79993666,
        ///         "state": "deployed",
        ///         "holder": [
        ///             {
        ///                 "address": "kaspa:qra0p5kyzeh54p37gqwfu...",
        ///                 "amount": "220000000000000"
        ///             }
        ///         ]
        ///     }
        /// }
        /// ```
        ///   

        #[derive(Debug, Deserialize)]
        pub struct TokenHolderResponse {
            pub message: String,
            pub result: TokenHolderResult,
        }

        #[serde_as]
        #[derive(Debug, Deserialize)]
        pub struct TokenHolderResult {
            pub tick: String,

            #[serde_as(as = "DisplayFromStr")]
            pub max: u128,

            #[serde_as(as = "DisplayFromStr")]
            pub lim: u128,

            #[serde_as(as = "DisplayFromStr")]
            pub dec: u64,

            #[serde_as(as = "DisplayFromStr")]
            #[serde(rename = "daas")]
            pub mint_start_daa_score: u64,

            #[serde_as(as = "DisplayFromStr")]
            #[serde(rename = "daae")]
            pub mint_end_daa_score: u64,

            #[serde_as(as = "DisplayFromStr")]
            pub minted: u128,

            #[serde_as(as = "DisplayFromStr")]
            #[serde(rename = "opScoreAdd")]
            pub op_score_add: u64,

            #[serde_as(as = "DisplayFromStr")]
            #[serde(rename = "opScoreMod")]
            pub op_score_mod: u64,

            pub state: State,

            pub holder: Vec<TokenHolder>,
        }

        #[serde_as]
        #[derive(Debug, Deserialize)]
        pub struct TokenHolder {
            pub address: String,
            #[serde_as(as = "DisplayFromStr")]
            pub amount: u128,
        }

        ///
        /// URL path: `//krc20/address/{address}/token/{tick}`
        ///
        /// ```json
        /// {
        ///     "message": "text",
        ///     "result": [
        ///         {
        ///             "tick": "text",
        ///             "balance": "text",
        ///             "locked": "0",
        ///             "dec": "text",
        ///             "opScoreMod": "79993666"
        ///         }
        ///     ]
        /// }
        /// ```
        ///

        #[derive(Debug, Deserialize)]
        pub struct TokenBalanceResponse {
            pub message: String,
            pub result: Vec<TokenBalance>,
        }

        #[derive(Debug, Deserialize)]
        #[serde_as]
        pub struct TokenBalance {
            pub tick: String,

            #[serde_as(as = "DisplayFromStr")]
            pub balance: u128,

            #[serde_as(as = "DisplayFromStr")]
            pub locked: u64,

            /// TODO - what is dec?
            #[serde_as(as = "DisplayFromStr")]
            pub dec: u64,

            #[serde_as(as = "DisplayFromStr")]
            #[serde(rename = "opScoreMod")]
            pub op_score_mod: u64,
        }

        ///
        /// URL path: `//krc20/oplist/{op}`
        ///
        /// ```json
        /// {
        ///     "message": "text",
        ///     "prev": "text",
        ///     "next": "text",
        ///     "result": [
        ///         {
        ///             "p": "KRC-20",
        ///             "op": "DEPLOY",
        ///             "tick": "KEKE",
        ///             "max": "2100000000000000",
        ///             "lim": "100000000000",
        ///             "dec": "8",
        ///             "daas": "78000000",
        ///             "daae": "88000000",
        ///             "amt": "2300000000",
        ///             "from": "kaspa:qra0p5ky...",
        ///             "to": "kaspa:qqabb6cz...",
        ///             "opScore": "779066550003",
        ///             "hashRev": "eb1482705b07af...",
        ///             "feeRev": "100010000",
        ///             "txAccept": "text",
        ///             "opAccept": "text",
        ///             "opError": "Insufficient fee",
        ///             "mtsAdd": "1712808987852",
        ///             "mtsMod": "1712808990016"
        ///         }
        ///     ]
        /// }
        /// ```

        #[derive(Debug, Deserialize)]
        pub struct TokenTransactionResponse {
            pub message: String,
            pub next: String,
            pub prev: String,
            pub result: Vec<TokenTransaction>,
        }

        #[serde_as]
        #[derive(Debug, Deserialize)]
        pub struct TokenTransaction {
            #[serde_as(as = "DisplayFromStr")]
            #[serde(rename = "p")]
            pub protocol: Protocol,

            pub op: String,

            pub tick: String,

            #[serde_as(as = "DisplayFromStr")]
            pub max: u128,

            #[serde(rename = "lim")]
            pub limit: u128,

            pub dec: u64,

            pub daas: u64,

            pub daae: u64,

            #[serde_as(as = "DisplayFromStr")]
            #[serde(rename = "amt")]
            pub amount: u128,

            pub from: String,

            pub to: String,

            #[serde_as(as = "DisplayFromStr")]
            #[serde(rename = "opScore")]
            pub op_score: u64,

            #[serde(rename = "hashRev")]
            pub hash_rev: Hash,

            #[serde(rename = "feeRev")]
            pub fee_rev: String,

            #[serde(rename = "txAccept")]
            pub tx_accept: String,

            #[serde(rename = "opAccept")]
            pub op_accept: String,

            #[serde(rename = "opError")]
            pub op_error: Option<String>,

            #[serde(rename = "mtsAdd")]
            pub mts_add: String,

            #[serde(rename = "mtsMod")]
            pub mts_mod: String,
        }

        ///
        /// URL path: `//krc20/op/{id}`
        ///
        /// ```json
        /// {
        ///     "message": "text",
        ///     "result": [
        ///         {
        ///             "p": "KRC-20",
        ///             "op": "DEPLOY",
        ///             "tick": "KEKE",
        ///             "max": "2100000000000000",
        ///             "lim": "100000000000",
        ///             "dec": "8",
        ///             "daas": "78000000",
        ///             "daae": "88000000",
        ///             "amt": "2300000000",
        ///             "from": "kaspa:qra0p5ky...",
        ///             "to": "kaspa:qqabb6cz...",
        ///             "opScore": "779066550003",
        ///             "hashRev": "eb1482705b07af...",
        ///             "feeRev": "100010000",
        ///             "txAccept": "text",
        ///             "opAccept": "text",
        ///             "opError": "Insufficient fee",
        ///             "mtsAdd": "1712808987852",
        ///             "mtsMod": "1712808990016"
        ///         }
        ///     ]
        /// }
        /// ```
        ///

        #[derive(Deserialize)]
        pub struct TokenTransactionIdResponse {
            pub message: String,
            pub result: Vec<TokenTransaction>,
        }
    }
}
