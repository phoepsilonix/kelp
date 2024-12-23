//! # kelp
//!
//! This is a porting from [jaconv(Python)](https://github.com/ikegami-yukino/jaconv).
mod conv_table;
mod convert;

pub use convert::h2z;
pub use convert::hira2hkata;
pub use convert::hira2kata;
pub use convert::kata2hira;
pub use convert::z2h;

/// Convert options
#[derive(Debug, Default, Clone, Copy)]
pub struct ConvOption<'a> {
    pub ascii: bool,
    pub digit: bool,
    pub ignore: &'a str,
    pub kana: bool,
}
