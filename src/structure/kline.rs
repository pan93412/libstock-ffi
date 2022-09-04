use safer_ffi::prelude::*;
use typed_builder::TypedBuilder;
pub use wmjtyd_libstock::data::kline::KlineStructure as RKlineStructure;
use wmjtyd_libstock::data::fields::SymbolPairField;

use crate::fields::{Exchange, MarketType, MessageType, Period, KlineIndicatorsField};
use crate::serializer::{construct_deserializer, construct_free_function, construct_serializer};

/// The structure of BBO.
#[derive(Clone, Debug)]
#[derive(TypedBuilder)]
#[derive_ReprC]
#[repr(C)]
pub struct KlineStructure {
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

    /// PERIOD
    pub period: Period,

    /// K 線指標
    pub indicator: KlineIndicatorsField,
}

impl TryFrom<KlineStructure> for RKlineStructure {
    type Error = anyhow::Error;

    fn try_from(value: KlineStructure) -> Result<Self, Self::Error> {
        Ok(
            Self::builder()
                .exchange_timestamp(value.exchange_timestamp as i64)
                .received_timestamp(value.received_timestamp.into())
                .exchange_type(value.exchange_type)
                .market_type(value.market_type)
                .message_type(value.message_type)
                .symbol(SymbolPairField::from_pair(value.pair.to_str()))
                .period(value.period)
                .indicator(value.indicator.clone().try_into()?)
                .build()
        )
    }
}

impl TryFrom<RKlineStructure> for KlineStructure {
    type Error = anyhow::Error;

    fn try_from(value: RKlineStructure) -> Result<Self, Self::Error> {
        Ok(
            Self::builder()
                .exchange_timestamp(value.exchange_timestamp.0)
                .received_timestamp(value.received_timestamp.0)
                .exchange_type(value.exchange_type)
                .market_type(value.market_type)
                .message_type(value.message_type)
                .pair(value.symbol.pair.try_into()?)
                .period(value.period.try_into()?)
                .indicator(value.indicator.try_into()?)
                .build()
        )
    }
}

impl TryFrom<&KlineStructure> for RKlineStructure {
    type Error = anyhow::Error;

    fn try_from(value: &KlineStructure) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}

construct_serializer!(serialize_kline, KlineStructure, RKlineStructure);
construct_deserializer!(deserialize_kline, KlineStructure, RKlineStructure);
construct_free_function!(free_kline, KlineStructure);
