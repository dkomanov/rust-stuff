#![feature(test)]

pub use crate::decode::{decode, decode_slice, DecodeError};
pub use crate::encode::{encode, encode_measter};

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pad: bool,
    url_safe: bool,
}

pub const STANDARD: Config = Config {
    pad: true,
    url_safe: false,
};

pub const STANDARD_NO_PAD: Config = Config {
    pad: false,
    url_safe: false,
};

pub const URL_SAFE: Config = Config {
    pad: true,
    url_safe: false,
};

pub const URL_SAFE_NO_PAD: Config = Config {
    pad: false,
    url_safe: false,
};

const PAD_BYTE: u8 = b'=';

mod encode;
mod decode;
mod tables;
