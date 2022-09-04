use safer_ffi::prelude::*;
use typed_builder::TypedBuilder;
pub use wmjtyd_libstock::data::bbo::BboStructure as RBboStructure;
use wmjtyd_libstock::data::fields::SymbolPairField;

use crate::fields::{Exchange, MarketType, MessageType, PriceDataField};
use crate::serializer::{construct_deserializer, construct_free_function, construct_serializer};

/// The structure of BBO.
#[derive(TypedBuilder)]
#[derive(Clone, Debug)]
#[derive_ReprC]
#[repr(C)]
pub struct BboStructure {
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

    /// 最優賣出報價資訊 (asks)
    pub asks: PriceDataField,

    /// 最優買入報價資訊 (bids)
    pub bids: PriceDataField,
}

impl TryFrom<&BboStructure> for RBboStructure {
    type Error = anyhow::Error;

    fn try_from(value: &BboStructure) -> Result<Self, Self::Error> {
        Ok(Self::builder()
            .exchange_timestamp(value.exchange_timestamp as i64)
            .received_timestamp(value.received_timestamp.into())
            .exchange_type(value.exchange_type)
            .market_type(value.market_type)
            .message_type(value.message_type)
            .symbol(SymbolPairField::from_pair(value.pair.to_str()))
            .asks(value.asks.clone().try_into()?)
            .bids(value.bids.clone().try_into()?)
            .build())
    }
}

impl TryFrom<&RBboStructure> for BboStructure {
    type Error = anyhow::Error;

    fn try_from(value: &RBboStructure) -> Result<Self, Self::Error> {
        Ok(Self::builder()
            .exchange_timestamp(value.exchange_timestamp.0)
            .received_timestamp(value.received_timestamp.0)
            .exchange_type(value.exchange_type)
            .market_type(value.market_type)
            .message_type(value.message_type)
            .pair(value.symbol.pair.clone().try_into()?)
            .asks(value.asks.clone().try_into()?)
            .bids(value.bids.clone().try_into()?)
            .build())
    }
}

impl TryFrom<RBboStructure> for BboStructure {
    type Error = anyhow::Error;

    fn try_from(value: RBboStructure) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<BboStructure> for RBboStructure {
    type Error = anyhow::Error;

    fn try_from(value: BboStructure) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

construct_serializer!(serialize_bbo, BboStructure, RBboStructure);
construct_deserializer!(deserialize_bbo, BboStructure, RBboStructure);
construct_free_function!(free_bbo, BboStructure);
