use anyhow::Context;
use ::safer_ffi::prelude::*;
use strum::FromRepr;
pub use wmjtyd_libstock::data::fields::PeriodField as RPeriodField;
use wmjtyd_libstock::data::serializer::{FieldDeserializer, FieldSerializer};

#[derive(Copy, Clone, FromRepr, Debug, PartialEq, Eq)]
#[derive_ReprC]
#[repr(u8)]
pub enum Period {
    /// 1 分鐘
    OneMinute = 1,
    /// 5 分鐘
    FiveMinutes = 2,
    /// 半小時（30 分鐘）
    HalfHour = 3,
    /// 1 小時
    OneHour = 4,
}

impl From<Period> for RPeriodField {
    fn from(p: Period) -> Self {
        RPeriodField::deserialize(&[p as u8]).unwrap()
    }
}

impl TryFrom<RPeriodField> for Period {
    type Error = anyhow::Error;

    fn try_from(p: RPeriodField) -> Result<Self, Self::Error> {
        let serialized = p.serialize().context("The RPeriodField input is not valid")?;

        Period::from_repr(serialized[0]).context("The Period is not implemented.")
    }
}

#[cfg(test)]
mod tests {
    use wmjtyd_libstock::data::fields::PeriodField as RPeriodField;

    use crate::fields::Period;

    #[test]
    fn test_period_convert() {
        const MAP: &[(Period, &str)] = &[
            (super::Period::OneMinute, "1m"),
            (super::Period::FiveMinutes, "5m"),
            (super::Period::HalfHour, "30m"),
            (super::Period::OneHour, "1h"),
        ];

        for (ffi_side, rust_side) in MAP {
            let rust_side = RPeriodField(String::from(*rust_side));
            assert_eq!(Period::try_from(rust_side).unwrap(), *ffi_side);
        }
    }

    #[test]
    fn test_rust_period_convert() {
        const MAP: &[(Period, &str)] = &[
            (super::Period::OneMinute, "1m"),
            (super::Period::FiveMinutes, "5m"),
            (super::Period::HalfHour, "30m"),
            (super::Period::OneHour, "1h"),
        ];

        for (ffi_side, rust_side) in MAP {
            let ffi_converted = RPeriodField::from(*ffi_side);
            assert_eq!(ffi_converted.0, *rust_side);
        }
    }
}
