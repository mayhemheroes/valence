#![no_main]
use libfuzzer_sys::fuzz_target;
use serde::{Deserialize, Serialize};
use valence::protocol::{Decode, Encode, NbtBridge};

const DEBUG: bool = false;

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
    ($ty:ty, $data:expr, $equality:expr) => {{
        let mut data = $data.clone();
        let x: Result<NbtBridge<$ty>, _> = Decode::decode(&mut data);
        if let Ok(t) = x {
            let mut bytes = Vec::new();
            Encode::encode(&t, &mut bytes).expect("a deserialized type should serialize");
            if DEBUG {
                dbg!(&bytes);
            }

            let des: NbtBridge<$ty> = Decode::decode(&mut bytes.as_slice())
                .expect("a serialized type should deserialize");
            if DEBUG {
                dbg!(&des);
            }

            if $equality {
                assert_eq!(t, des, "roundtripped object changed");
            }
        }
    }};
}

fuzz_target!(|data: &[u8]| {
    round_trip!(bool, data, true);
    round_trip!(i8, data, true);
    round_trip!(i16, data, true);
    round_trip!(i32, data, true);
    round_trip!(i64, data, true);
    round_trip!(i128, data, true);
    round_trip!(u8, data, true);
    round_trip!(u16, data, true);
    round_trip!(u32, data, true);
    round_trip!(u64, data, true);
    round_trip!(u128, data, true);
    round_trip!(f32, data, false);
    round_trip!(f64, data, false);
    round_trip!(char, data, true);
    round_trip!((), data, true);
    round_trip!(PlainEnum, data, true);
    round_trip!(Enum, data, true);
    round_trip!(FloatEnum, data, false);
    round_trip!(Struct, data, true);
    round_trip!(FloatStruct, data, false);
});
