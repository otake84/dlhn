pub mod de;
pub mod ser;

const UNIT_CODE: u8 = 0;
const OPTIONAL_CODE: u8 = 1;
const BOOLEAN_CODE: u8 = 2;
const UINT8_CODE: u8 = 3;
const UINT16_CODE: u8 = 4;
const UINT32_CODE: u8 = 5;
const UINT64_CODE: u8 = 6;
// const UINT128_CODE: u8 = 7;
const INT8_CODE: u8 = 8;
const INT16_CODE: u8 = 9;
const INT32_CODE: u8 = 10;
const INT64_CODE: u8 = 11;
// const INT128_CODE: u8 = 12;
const FLOAT32_CODE: u8 = 13;
const FLOAT64_CODE: u8 = 14;
const BIG_UINT_CODE: u8 = 15;
const BIG_INT_CODE: u8 = 16;
const BIG_DECIMAL_CODE: u8 = 17;
const STRING_CODE: u8 = 18;
const BINARY_CODE: u8 = 19;
const ARRAY_CODE: u8 = 20;
const TUPLE_CODE: u8 = 21;
const STRUCT_CODE: u8 = 22;
const MAP_CODE: u8 = 23;
const ENUM_CODE: u8 = 24;
const DATE_CODE: u8 = 25;
const DATETIME_CODE: u8 = 26;

#[derive(Clone, Debug, PartialEq)]
pub enum Header {
    Unit,
    Optional(Box<Header>),
    Boolean,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    // UInt128,
    Int8,
    Int16,
    Int32,
    Int64,
    // Int128,
    Float32,
    Float64,
    BigUInt,
    BigInt,
    BigDecimal,
    String,
    Binary,
    Array(Box<Header>),
    Tuple(Vec<Header>),
    Struct(Vec<Header>),
    Map(Box<Header>),
    Enum(Vec<Header>),
    Date,
    DateTime,
}
