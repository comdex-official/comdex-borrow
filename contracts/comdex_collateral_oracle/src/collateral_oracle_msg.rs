use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use cswap::asset::AssetInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub mint_contract: String,
    pub base_denom: String,
    pub mirror_oracle: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
        mint_contract: Option<String>,
        base_denom: Option<String>,
        mirror_oracle: Option<String>,
    },
    RegisterCollateralAsset {
        asset: AssetInfo,
        price_source: SourceType,
        multiplier: Decimal,
    },
    RevokeCollateralAsset {
        asset: AssetInfo,
    },
    UpdateCollateralPriceSource {
        asset: AssetInfo,
        price_source: SourceType,
    },
    UpdateCollateralMultiplier {
        asset: AssetInfo,
        multiplier: Decimal,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    CollateralPrice {
        asset: String,
        block_height: Option<u64>,
    },
    CollateralAssetInfo {
        asset: String,
    },
    CollateralAssetInfos {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
    pub mint_contract: String,
    pub base_denom: String,
    pub mirror_oracle: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralPriceResponse {
    pub asset: String,
    pub rate: Decimal,
    pub last_updated: u64,
    pub multiplier: Decimal,
    pub is_revoked: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralInfoResponse {
    pub asset: String,
    pub multiplier: Decimal,
    pub source_type: String,
    pub is_revoked: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralInfosResponse {
    pub collaterals: Vec<CollateralInfoResponse>,
}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    MirrorOracle {},
    /*Native {
        native_denom: String,
    },*/
}

impl fmt::Display for SourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SourceType::MirrorOracle { .. } => write!(f, "mirror_oracle"),
            //SourceType::Native { .. } => write!(f, "native"),
        }
    }
}
