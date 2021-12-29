#![forbid(unsafe_code)]

pub mod big_decimal;
pub mod big_int;
pub mod big_uint;
pub mod body;
pub mod date;
pub mod date_time;
pub mod de;
pub mod format;
pub mod header;
pub(crate) mod leb128;
pub(crate) mod prefix_varint;
pub mod ser;
pub(crate) mod zigzag;
