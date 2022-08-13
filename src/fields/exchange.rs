use ::safer_ffi::prelude::*;
use strum::FromRepr;
pub use wmjtyd_libstock::data::fields::exchange_type::{
    Exchange as RExchange,
};

#[derive(Copy, Clone, FromRepr, Debug)]
#[derive_ReprC]
#[repr(u16)]
pub enum Exchange {
    Crypto = 1,
    Ftx = 2,
    Binance = 3,
    Huobi = 8,
    Kucoin = 10,
    Okx = 11,
}

impl From<RExchange> for Exchange {
    fn from(src: RExchange) -> Self {
        Self::from_repr(src as u16).unwrap()
    }
}

impl From<Exchange> for RExchange {
    fn from(src: Exchange) -> Self {
        Self::from_repr(src as usize).unwrap()
    }
}
