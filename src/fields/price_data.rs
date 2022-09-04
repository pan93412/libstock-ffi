use safer_ffi::prelude::*;
use anyhow::Context;
use wmjtyd_libstock::data::fields::DecimalField;
pub use wmjtyd_libstock::data::fields::PriceDataField as RPriceDataField;

#[derive(Clone, Debug)]
#[derive_ReprC]
#[repr(C)]
pub struct PriceDataField {
    /// 價格（64 位浮點數字串）
    pub price: char_p::Box,

    /// 基本量（64 位浮點數字串）
    pub quantity_base: char_p::Box,
}

impl TryFrom<&RPriceDataField> for PriceDataField {
    type Error = anyhow::Error;

    fn try_from(src: &RPriceDataField) -> Result<Self, Self::Error> {
        Ok(Self {
            price: src.price.to_string().try_into()?,
            quantity_base: src.quantity_base.to_string().try_into()?,
        })
    }
}

impl TryFrom<&PriceDataField> for RPriceDataField {
    type Error = anyhow::Error;

    fn try_from(src: &PriceDataField) -> Result<Self, Self::Error> {
        Ok(Self {
            price: DecimalField::try_from(src.price.to_str()).context("Not a valid decimal format")?,
            quantity_base: DecimalField::try_from(src.quantity_base.to_str()).context("Not a valid decimal format")?,
        })
    }
}

impl TryFrom<RPriceDataField> for PriceDataField {
    type Error = anyhow::Error;

    fn try_from(src: RPriceDataField) -> Result<Self, Self::Error> {
        Self::try_from(&src)
    }
}

impl TryFrom<PriceDataField> for RPriceDataField {
    type Error = anyhow::Error;

    fn try_from(src: PriceDataField) -> Result<Self, Self::Error> {
        Self::try_from(&src)
    }
}
