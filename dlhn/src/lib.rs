#![forbid(unsafe_code)]

pub mod body;
pub mod date;
pub mod de;
pub mod format;
pub mod header;
pub(crate) mod leb128;
pub mod ser;
pub(crate) mod zigzag;
