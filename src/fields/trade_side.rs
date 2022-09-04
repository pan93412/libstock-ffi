use safer_ffi::prelude::*;
pub use wmjtyd_libstock::data::fields::trade_side::TradeSide as RTradeSide;
pub use wmjtyd_libstock::data::fields::trade_side::TradeSideField as RTradeSideField;

#[derive(Copy, Clone, Debug)]
#[derive_ReprC]
#[repr(u8)]
pub enum TradeSide {
    Buy = 1,
    Sell = 2
}

impl From<RTradeSide> for TradeSide {
    fn from(value: RTradeSide) -> Self {
        match value {
            RTradeSide::Buy => Self::Buy,
            RTradeSide::Sell => Self::Sell
        }
    }
}

impl From<TradeSide> for RTradeSide {
    fn from(value: TradeSide) -> Self {
        match value {
            TradeSide::Buy => Self::Buy,
            TradeSide::Sell => Self::Sell
        }
    }
}
