#![no_main]
use libfuzzer_sys::fuzz_target;

use serde::{Deserialize, Serialize};

use postcard::from_bytes;
use postcard::from_bytes_cobs;
use postcard::to_stdvec;
use postcard::to_stdvec_cobs;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum PlainEnum {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Enum {
    A(u8),
    B(()),
    C(Vec<PlainEnum>),
    D(i128),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum FloatEnum {
    A(Enum),
    E(Option<f32>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Struct {
    _a: (),
    _b: u8,
    _c: Vec<Enum>,
    _d: (u128, i8, (), PlainEnum, String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct FloatStruct {
    _a: Struct,
    _b: f64,
}

macro_rules! round_trip {
    ($t:expr, $ty:ty, $to_bytes:ident, $from_bytes:ident, $equality:expr) => {{
        let mut ser = $to_bytes(&$t).expect("a deserialized type should serialize");
        #[cfg(feature = "debug")]
        dbg!(&ser);

        let des: $ty = $from_bytes(&mut ser).expect("a serialized type should deserialize");
        #[cfg(feature = "debug")]
        dbg!(&des);

        if $equality {
            assert_eq!($t, des, "roundtripped object changed");
        }
    }};
}

macro_rules! from_bytes {
    ($ty:ty, $data:expr, $equality:expr) => {{
        let cobs_data = $data.to_vec();

        // Normal T

        let x: Result<$ty, _> = from_bytes(&$data);
        if let Ok(t) = x {
            round_trip!(t, $ty, to_stdvec, from_bytes, $equality);
            round_trip!(t, $ty, to_stdvec_cobs, from_bytes_cobs, $equality);
        }
        let mut data = cobs_data.clone();
        let x: Result<$ty, _> = from_bytes_cobs(&mut data);
        if let Ok(t) = x {
            round_trip!(t, $ty, to_stdvec, from_bytes, $equality);
            round_trip!(t, $ty, to_stdvec_cobs, from_bytes_cobs, $equality);
        }

        // Option<T>

        let x: Result<Option<$ty>, _> = from_bytes(&$data);
        if let Ok(t) = x {
            round_trip!(t, Option<$ty>, to_stdvec, from_bytes, $equality);
            round_trip!(t, Option<$ty>, to_stdvec_cobs, from_bytes_cobs, $equality);
        }
        let mut data = cobs_data.clone();
        let x: Result<Option<$ty>, _> = from_bytes_cobs(&mut data);
        if let Ok(t) = x {
            round_trip!(t, Option<$ty>, to_stdvec, from_bytes, $equality);
            round_trip!(t, Option<$ty>, to_stdvec_cobs, from_bytes_cobs, $equality);
        }

        // Vec<T>

        let x: Result<Vec<$ty>, _> = from_bytes(&$data);
        if let Ok(t) = x {
            round_trip!(t, Vec<$ty>, to_stdvec, from_bytes, $equality);
            round_trip!(t, Vec<$ty>, to_stdvec_cobs, from_bytes_cobs, $equality);
        }
        let mut data = cobs_data;
        let x: Result<Vec<$ty>, _> = from_bytes_cobs(&mut data);
        if let Ok(t) = x {
            round_trip!(t, Vec<$ty>, to_stdvec, from_bytes, $equality);
            round_trip!(t, Vec<$ty>, to_stdvec_cobs, from_bytes_cobs, $equality);
        }
    }};
}

fuzz_target!(|data: &[u8]| {
    from_bytes!(bool, data, true);
    from_bytes!(i8, data, true);
    from_bytes!(i16, data, true);
    from_bytes!(i32, data, true);
    from_bytes!(i64, data, true);
    from_bytes!(i128, data, true);
    from_bytes!(u8, data, true);
    from_bytes!(u16, data, true);
    from_bytes!(u32, data, true);
    from_bytes!(u64, data, true);
    from_bytes!(u128, data, true);
    from_bytes!(f32, data, false);
    from_bytes!(f64, data, false);
    from_bytes!(char, data, true);
    from_bytes!(&str, data, true);
    from_bytes!((), data, true);
    from_bytes!(PlainEnum, data, true);
    from_bytes!(Enum, data, true);
    from_bytes!(FloatEnum, data, false);
    from_bytes!(Struct, data, true);
    from_bytes!(FloatStruct, data, false);
});
