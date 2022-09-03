use safer_ffi::prelude::*;
pub use wmjtyd_libstock::data::fields::message_type::MessageType as RMessageType;
pub use wmjtyd_libstock::data::fields::message_type::MessageTypeField as RMessageTypeField;

#[derive(Copy, Clone, Debug)]
#[derive_ReprC]
#[repr(u8)]
pub enum MessageType {
    /// All other messages
    Other = 0,
    /// tick-by-tick trade messages
    Trade = 1,
    /// Best bid and ask
    BBO = 2,
    /// Level2 top K snapshots from websocket
    L2TopK = 3,
    /// Level2 snapshot from RESTful API
    L2Snapshot = 4,
    /// Incremental level2 orderbook updates
    L2Event = 5,
    /// Level3 snapshot from RESTful API
    L3Snapshot = 6,
    /// Incremental level3 orderbook updates
    L3Event = 7,
    /// 24hr rolling window ticker
    Ticker = 8,
    /// OHLCV candlestick
    Candlestick = 9,
    /// Open interest
    OpenInterest = 10,
    /// Funding rate
    FundingRate = 11,
    /// Long/short ratio
    LongShortRatio = 12,
    /// Taker buy/sell volume
    TakerVolume = 13,
}

impl From<MessageType> for RMessageType {
    fn from(mt: MessageType) -> Self {
        match mt {
            MessageType::Other => RMessageType::Other,
            MessageType::Trade => RMessageType::Trade,
            MessageType::BBO => RMessageType::BBO,
            MessageType::L2TopK => RMessageType::L2TopK,
            MessageType::L2Snapshot => RMessageType::L2Snapshot,
            MessageType::L2Event => RMessageType::L2Event,
            MessageType::L3Snapshot => RMessageType::L3Snapshot,
            MessageType::L3Event => RMessageType::L3Event,
            MessageType::Ticker => RMessageType::Ticker,
            MessageType::Candlestick => RMessageType::Candlestick,
            MessageType::OpenInterest => RMessageType::OpenInterest,
            MessageType::FundingRate => RMessageType::FundingRate,
            MessageType::LongShortRatio => RMessageType::LongShortRatio,
            MessageType::TakerVolume => RMessageType::TakerVolume,
        }
    }
}

impl From<RMessageType> for MessageType {
    fn from(mt: RMessageType) -> Self {
        match mt {
            RMessageType::Other => MessageType::Other,
            RMessageType::Trade => MessageType::Trade,
            RMessageType::BBO => MessageType::BBO,
            RMessageType::L2TopK => MessageType::L2TopK,
            RMessageType::L2Snapshot => MessageType::L2Snapshot,
            RMessageType::L2Event => MessageType::L2Event,
            RMessageType::L3Snapshot => MessageType::L3Snapshot,
            RMessageType::L3Event => MessageType::L3Event,
            RMessageType::Ticker => MessageType::Ticker,
            RMessageType::Candlestick => MessageType::Candlestick,
            RMessageType::OpenInterest => MessageType::OpenInterest,
            RMessageType::FundingRate => MessageType::FundingRate,
            RMessageType::LongShortRatio => MessageType::LongShortRatio,
            RMessageType::TakerVolume => MessageType::TakerVolume,
        }
    }
}

impl From<MessageType> for RMessageTypeField {
    fn from(mt: MessageType) -> Self {
        Self(mt.into())
    }
}

impl From<RMessageTypeField> for MessageType {
    fn from(mt: RMessageTypeField) -> Self {
        Self::from(mt.0)
    }
}
