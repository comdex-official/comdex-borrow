use std::str::FromStr;
use crate::state::Config;
use cosmwasm_bignumber::{Uint256};
use cosmwasm_std::{
    to_binary, Decimal, Deps, QuerierWrapper, QueryRequest, StdError, StdResult, Uint128, WasmQuery,
};
use crate::collateral_oracle_msg::SourceType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceQueryMsg {
    Price {
        base_asset: String,
        quote_asset: String,
    },
    Pool {},
    GetReferenceData {
        base_symbol: String,
        quote_symbol: String,
    },
    EpochState {
        block_heigth: Option<u64>,
        distributed_interest: Option<Uint256>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TerraOracleResponse {
    // oracle queries returns rate
    pub rate: Decimal,
    pub last_updated_base: u64,
}

#[allow(clippy::ptr_arg)]
pub fn query_price(
    deps: Deps,
    config: &Config,
    asset: &String,
    block_height: Option<u64>,
    price_source: &SourceType,
) -> StdResult<(Decimal, u64)> {
    match price_source {
        SourceType::MirrorOracle {} => {
            let res: TerraOracleResponse =
                deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr: deps.api.addr_humanize(&config.mirror_oracle)?.to_string(),
                    msg: to_binary(&SourceQueryMsg::Price {
                        base_asset: asset.to_string(),
                        quote_asset: config.base_denom.clone(),
                    })
                    .unwrap(),
                }))?;

            Ok((res.rate, res.last_updated_base))
        }
        /*SourceType::Native { native_denom } => {
            let rate: Decimal = query_native_rate(
                &deps.querier,
                native_denom.clone(),
                config.base_denom.clone(),
            )?;

            Ok((rate, u64::MAX))
        }*/
    }
}

/// Parses a uint that contains the price multiplied by 1e18
fn parse_band_rate(uint_rate: Uint128) -> StdResult<Decimal> {
    // manipulate the uint as a string to prevent overflow
    let mut rate_uint_string: String = uint_rate.to_string();

    let uint_len = rate_uint_string.len();
    if uint_len > 18 {
        let dec_point = rate_uint_string.len() - 18;
        rate_uint_string.insert(dec_point, '.');
    } else {
        let mut prefix: String = "0.".to_owned();
        let dec_zeros = 18 - uint_len;
        for _ in 0..dec_zeros {
            prefix.push('0');
        }
        rate_uint_string = prefix + rate_uint_string.as_str();
    }

    Decimal::from_str(rate_uint_string.as_str())
}

/*fn query_native_rate(
    querier: &QuerierWrapper,
    base_denom: String,
    quote_denom: String,
) -> StdResult<Decimal> {
    let terra_querier = TerraQuerier::new(querier);
    let res: ExchangeRatesResponse =
        terra_querier.query_exchange_rates(base_denom, vec![quote_denom])?;

    Ok(res.exchange_rates[0].exchange_rate)
}*/

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_parse_band_rate() {
        let rate_dec_1: Decimal =
            parse_band_rate(Uint128::from(3493968700000000000000u128)).unwrap();
        assert_eq!(
            rate_dec_1,
            Decimal::from_str("3493.968700000000000000").unwrap()
        );

        let rate_dec_2: Decimal = parse_band_rate(Uint128::from(1234u128)).unwrap();
        assert_eq!(
            rate_dec_2,
            Decimal::from_str("0.000000000000001234").unwrap()
        );

        let rate_dec_3: Decimal = parse_band_rate(Uint128::from(100000000000000001u128)).unwrap();
        assert_eq!(
            rate_dec_3,
            Decimal::from_str("0.100000000000000001").unwrap()
        );
    }
}
