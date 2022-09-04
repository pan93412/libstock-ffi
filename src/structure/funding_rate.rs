use safer_ffi::prelude::*;
use typed_builder::TypedBuilder;
pub use wmjtyd_libstock::data::funding_rate::FundingRateStructure as RFundingRateStructure;
use wmjtyd_libstock::data::fields::{SymbolPairField, DecimalField};

use crate::fields::{Exchange, MarketType, MessageType};
use crate::serializer::{construct_deserializer, construct_free_function, construct_serializer};

/// The structure of BBO.
#[derive(Clone, Debug)]
#[derive(TypedBuilder)]
#[derive_ReprC]
#[repr(C)]
pub struct FundingRateStructure {
    /// 交易所時間戳（毫秒）
    pub exchange_timestamp: u64,

    /// 收到時間戳（毫秒）
    pub received_timestamp: u64,

    /// 交易所類型 (EXCHANGE)
    #[builder(setter(into))]
    pub exchange_type: Exchange,

    /// 市場類型 (MARKET_TYPE)
    #[builder(setter(into))]
    pub market_type: MarketType,

    /// 訊息類型 (MESSAGE_TYPE)
    #[builder(setter(into))]
    pub message_type: MessageType,

    /// Pair，如 `BTC/USDT`
    pub pair: char_p::Box,

    /// Funding rate（64 位浮點數字串）
    #[builder(setter(into))]
    pub funding_rate: char_p::Box,

    /// Funding time（毫秒）
    #[builder(setter(into))]
    pub funding_time: u64,

    /// Estimated rate（64 位浮點數字串）
    #[builder(setter(into))]
    pub estimated_rate: char_p::Box,
}

impl TryFrom<&FundingRateStructure> for RFundingRateStructure {
    type Error = anyhow::Error;

    fn try_from(value: &FundingRateStructure) -> Result<Self, Self::Error> {
        Ok(
            Self::builder()
                .exchange_timestamp(value.exchange_timestamp as i64)
                .received_timestamp(value.received_timestamp.into())
                .exchange_type(value.exchange_type)
                .market_type(value.market_type)
                .message_type(value.message_type)
                .symbol(SymbolPairField::from_pair(value.pair.to_str()))
                .funding_rate(DecimalField::try_from(value.funding_rate.to_str())?)
                .funding_time(value.funding_time)
                .estimated_rate(DecimalField::try_from(value.estimated_rate.to_str())?)
                .build()
        )
    }
}

impl TryFrom<&RFundingRateStructure> for FundingRateStructure {
    type Error = anyhow::Error;

    fn try_from(value: &RFundingRateStructure) -> Result<Self, Self::Error> {
        Ok(
            Self::builder()
                .exchange_timestamp(value.exchange_timestamp.0)
                .received_timestamp(value.received_timestamp.0)
                .exchange_type(value.exchange_type)
                .market_type(value.market_type)
                .message_type(value.message_type)
                .pair(value.symbol.pair.clone().try_into()?)
                .funding_rate(char_p::Box::try_from(value.funding_rate.to_string())?)
                .funding_time(value.funding_time.0)
                .estimated_rate(char_p::Box::try_from(value.estimated_rate.to_string())?)
                .build()
        )
    }
}

impl TryFrom<FundingRateStructure> for RFundingRateStructure {
    type Error = anyhow::Error;

    fn try_from(value: FundingRateStructure) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<RFundingRateStructure> for FundingRateStructure {
    type Error = anyhow::Error;

    fn try_from(value: RFundingRateStructure) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

construct_serializer!(serialize_funding_rate, FundingRateStructure, RFundingRateStructure);
construct_deserializer!(deserialize_funding_rate, FundingRateStructure, RFundingRateStructure);
construct_free_function!(free_funding_rate, FundingRateStructure);
