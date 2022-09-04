use safer_ffi::prelude::*;
use wmjtyd_libstock::data::fields::DecimalField;
use anyhow::Context;
pub use wmjtyd_libstock::data::fields::KlineIndicatorsField as RKlineIndicatorsField;

#[derive(Clone, Debug)]
#[derive_ReprC]
#[repr(C)]
pub struct KlineIndicatorsField {
    /// 開盤價（32 位浮點數字串）
    ///
    /// 開市後，第一筆成交的價格。
    pub open: char_p::Box,

    /// 最高價（32 位浮點數字串）
    ///
    /// 期間內，買賣的最高價。
    pub high: char_p::Box,

    /// 最低價（32 位浮點數字串）
    ///
    /// 期間內，買賣的最低價。
    pub low: char_p::Box,

    /// 收盤價（32 位浮點數字串）
    ///
    /// 閉市前，最後一筆成交的價格。
    pub close: char_p::Box,

    /// 交易量（64 位浮點數字串）
    pub volume: char_p::Box,
}

impl TryFrom<&RKlineIndicatorsField> for KlineIndicatorsField {
    type Error = anyhow::Error;

    fn try_from(src: &RKlineIndicatorsField) -> Result<Self, Self::Error> {
        Ok(Self {
            open: src.open.to_string().try_into()?,
            high: src.high.to_string().try_into()?,
            low: src.low.to_string().try_into()?,
            close: src.close.to_string().try_into()?,
            volume: src.volume.to_string().try_into()?,
        })
    }
}

impl TryFrom<&KlineIndicatorsField> for RKlineIndicatorsField {
    type Error = anyhow::Error;

    fn try_from(src: &KlineIndicatorsField) -> Result<Self, Self::Error> {
        fn converter<const L: usize>(src: &char_p::Box) -> Result<DecimalField<L>, anyhow::Error> {
            DecimalField::<L>::try_from(src.to_str()).context("Not a valid decimal format")
        }

        Ok(Self {
            open: converter(&src.open)?,
            high: converter(&src.high)?,
            low: converter(&src.low)?,
            close: converter(&src.close)?,
            volume: converter(&src.volume)?,
        })
    }
}

impl TryFrom<RKlineIndicatorsField> for KlineIndicatorsField {
    type Error = anyhow::Error;

    fn try_from(src: RKlineIndicatorsField) -> Result<Self, Self::Error> {
        Self::try_from(&src)
    }
}

impl TryFrom<KlineIndicatorsField> for RKlineIndicatorsField {
    type Error = anyhow::Error;

    fn try_from(src: KlineIndicatorsField) -> Result<Self, Self::Error> {
        Self::try_from(&src)
    }
}
