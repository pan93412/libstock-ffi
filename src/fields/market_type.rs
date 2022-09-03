use safer_ffi::prelude::*;
pub use wmjtyd_libstock::data::fields::market_type::MarketType as RMarketType;
pub use wmjtyd_libstock::data::fields::market_type::MarketTypeField as RMarketTypeField;

#[derive_ReprC]
#[repr(u8)]
pub enum MarketType {
    Unknown = 0,
    Spot = 1,
    LinearFuture = 2,
    InverseFuture = 3,
    LinearSwap = 4,
    InverseSwap = 5,

    EuropeanOption = 6,

    QuantoFuture = 7,
    QuantoSwap = 8,
}

impl From<MarketType> for RMarketType {
    fn from(mt: MarketType) -> Self {
        match mt {
            MarketType::Unknown => RMarketType::Unknown,
            MarketType::Spot => RMarketType::Spot,
            MarketType::LinearFuture => RMarketType::LinearFuture,
            MarketType::InverseFuture => RMarketType::InverseFuture,
            MarketType::LinearSwap => RMarketType::LinearSwap,
            MarketType::InverseSwap => RMarketType::InverseSwap,
            MarketType::EuropeanOption => RMarketType::EuropeanOption,
            MarketType::QuantoFuture => RMarketType::QuantoFuture,
            MarketType::QuantoSwap => RMarketType::QuantoSwap,
        }
    }
}

impl From<RMarketType> for MarketType {
    fn from(mt: RMarketType) -> Self {
        match mt {
            RMarketType::Unknown => MarketType::Unknown,
            RMarketType::Spot => MarketType::Spot,
            RMarketType::LinearFuture => MarketType::LinearFuture,
            RMarketType::InverseFuture => MarketType::InverseFuture,
            RMarketType::LinearSwap => MarketType::LinearSwap,
            RMarketType::InverseSwap => MarketType::InverseSwap,
            RMarketType::EuropeanOption => MarketType::EuropeanOption,
            RMarketType::QuantoFuture => MarketType::QuantoFuture,
            RMarketType::QuantoSwap => MarketType::QuantoSwap,
            _ => MarketType::Unknown,
        }
    }
}

impl From<MarketType> for RMarketTypeField {
    fn from(mt: MarketType) -> Self {
        Self(mt.into())
    }
}

impl From<RMarketTypeField> for MarketType {
    fn from(mt: RMarketTypeField) -> Self {
        Self::from(mt.0)
    }
}
