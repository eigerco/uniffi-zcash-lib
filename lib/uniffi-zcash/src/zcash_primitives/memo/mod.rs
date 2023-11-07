mod memo_bytes;
pub use self::memo_bytes::*;

use zcash_primitives::memo::Memo;
use zcash_primitives::memo::MemoBytes;

pub enum ZcashMemo {
    /// An empty memo field.
    Empty,
    /// A memo field containing a UTF-8 string.
    Text { v: String },
    /// Some unknown memo format from ✨*the future*✨ that we can't parse.
    Future { v: Vec<u8> },
    /// A memo field containing arbitrary bytes.
    Arbitrary { v: Vec<u8> }, // max 512
}

use std::str::FromStr;

impl From<ZcashMemo> for Memo {
    fn from(e: ZcashMemo) -> Self {
        match e {
            ZcashMemo::Empty => Self::Empty,
            // NOTE an occasion to handle better errors here
            ZcashMemo::Text { v } => Memo::from_str(&v).unwrap(),
            ZcashMemo::Future { v } => Self::Future(MemoBytes::from_bytes(&v[..]).unwrap()),
            ZcashMemo::Arbitrary { v } => {
                let boxed_slice: Box<[u8]> = v.into_boxed_slice();
                let arr: Box<[u8; 511]> = boxed_slice.try_into().unwrap();
                Self::Arbitrary(arr)
            }
        }
    }
}

impl From<Memo> for ZcashMemo {
    fn from(e: Memo) -> Self {
        match e {
            Memo::Empty => Self::Empty,
            Memo::Text(v) => Self::Text { v: v.to_string() },
            Memo::Future(v) => Self::Future {
                v: v.as_array().to_vec(),
            },
            Memo::Arbitrary(v) => Self::Arbitrary {
                v: (v as Box<[_]>).into_vec(),
            },
        }
    }
}
