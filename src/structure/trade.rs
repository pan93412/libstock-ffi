use safer_ffi::prelude::*;
use typed_builder::TypedBuilder;
pub use wmjtyd_libstock::data::trade::TradeStructure as RTradeStructure;
use wmjtyd_libstock::data::fields::{SymbolPairField, TradeSideField};

use crate::fields::{Exchange, MarketType, MessageType, PriceDataField, TradeSide};
use crate::serializer::{construct_deserializer, construct_free_function, construct_serializer};

/// The structure of BBO.
#[derive(TypedBuilder)]
#[derive(Clone, Debug)]
#[derive_ReprC]
#[repr(C)]
pub struct TradeStructure {
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

    /// 交易方向 (Trade side)
    ///
    /// Buy or Sell?
    #[builder(setter(into))]
    pub trade_side: TradeSide,

    /// 交易價格資訊
    pub trade_price: PriceDataField,
}

impl TryFrom<&TradeStructure> for RTradeStructure {
    type Error = anyhow::Error;

    fn try_from(value: &TradeStructure) -> Result<Self, Self::Error> {
        Ok(Self::builder()
            .exchange_timestamp(value.exchange_timestamp as i64)
            .received_timestamp(value.received_timestamp.into())
            .exchange_type(value.exchange_type)
            .market_type(value.market_type)
            .message_type(value.message_type)
            .symbol(SymbolPairField::from_pair(value.pair.to_str()))
            .trade_side(TradeSideField(value.trade_side.into()))
            .trace_price(value.trade_price.clone().try_into()?)
            .build())
    }
}

impl TryFrom<&RTradeStructure> for TradeStructure {
    type Error = anyhow::Error;

    fn try_from(value: &RTradeStructure) -> Result<Self, Self::Error> {
        Ok(Self::builder()
            .exchange_timestamp(value.exchange_timestamp.0)
            .received_timestamp(value.received_timestamp.0)
            .exchange_type(value.exchange_type)
            .market_type(value.market_type)
            .message_type(value.message_type)
            .pair(value.symbol.pair.clone().try_into()?)
            .trade_side(value.trade_side.0)
            .trade_price(value.trace_price.clone().try_into()?)
            .build())
    }
}

impl TryFrom<RTradeStructure> for TradeStructure {
    type Error = anyhow::Error;

    fn try_from(value: RTradeStructure) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<TradeStructure> for RTradeStructure {
    type Error = anyhow::Error;

    fn try_from(value: TradeStructure) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

construct_serializer!(serialize_trade, TradeStructure, RTradeStructure);
construct_deserializer!(deserialize_trade, TradeStructure, RTradeStructure);
construct_free_function!(free_trade, TradeStructure);
