use dlhn::{body::Body, ser::Serializer};
use iai::main;
use serde::Serialize;
use serde_bytes::ByteBuf;
use std::collections::BTreeMap;

fn serialize_unit() -> Vec<u8> {
    serialize(Body::Unit)
}

fn serialize_optional() -> Vec<u8> {
    serialize(Body::Optional(Some(Box::new(Body::UInt8(u8::MAX)))))
}

fn serialize_bool() -> Vec<u8> {
    serialize(Body::Boolean(true))
}

fn serialize_u8() -> Vec<u8> {
    serialize(Body::UInt8(u8::MAX))
}

fn serialize_u16() -> Vec<u8> {
    serialize(Body::UInt16(u16::MAX))
}

fn serialize_u32() -> Vec<u8> {
    serialize(Body::UInt32(u32::MAX))
}

fn serialize_u64() -> Vec<u8> {
    serialize(Body::UInt64(u64::MAX))
}

// fn serialize_u128() -> Vec<u8> {
//     serialize(Body::UInt128(u128::MAX))
// }

fn serialize_i8() -> Vec<u8> {
    serialize(Body::Int8(i8::MAX))
}

fn serialize_i16() -> Vec<u8> {
    serialize(Body::Int16(i16::MAX))
}

fn serialize_i32() -> Vec<u8> {
    serialize(Body::Int32(i32::MAX))
}

fn serialize_i64() -> Vec<u8> {
    serialize(Body::Int64(i64::MAX))
}

// fn serialize_i128() -> Vec<u8> {
//     serialize(Body::Int128(i128::MAX))
// }

fn serialize_array() -> Vec<u8> {
    serialize(Body::Array(vec![Body::Boolean(true), Body::Boolean(false)]))
}

fn serialize_tuple() -> Vec<u8> {
    serialize(Body::Tuple(vec![Body::Boolean(true), Body::Boolean(false)]))
}

fn serialize_struct() -> Vec<u8> {
    serialize(Body::Struct(vec![
        Body::Boolean(true),
        Body::Boolean(false),
    ]))
}

fn serialize_map() -> Vec<u8> {
    serialize(Body::Map({
        let mut buf = BTreeMap::new();
        buf.insert("a".to_string(), Body::Boolean(true));
        buf.insert("b".to_string(), Body::Boolean(false));
        buf
    }))
}

fn serialize_enum() -> Vec<u8> {
    serialize(Body::Enum(0, Box::new(Body::Boolean(true))))
}

fn serialize_binary() -> Vec<u8> {
    serialize(Body::Binary(ByteBuf::from(vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93,
        94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
        131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148,
        149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166,
        167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184,
        185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202,
        203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
        221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238,
        239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
    ])))
}

fn serialize<T: Serialize>(v: T) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    v.serialize(&mut serializer).unwrap();
    buf
}

main!(
    serialize_unit,
    serialize_optional,
    serialize_bool,
    serialize_u8,
    serialize_u16,
    serialize_u32,
    serialize_u64,
    // serialize_u128,
    serialize_i8,
    serialize_i16,
    serialize_i32,
    serialize_i64,
    // serialize_i128,
    serialize_array,
    serialize_tuple,
    serialize_struct,
    serialize_map,
    serialize_enum,
    serialize_binary,
);
