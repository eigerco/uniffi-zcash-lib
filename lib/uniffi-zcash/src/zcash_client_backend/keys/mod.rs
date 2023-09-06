mod era;
pub use self::era::*;

mod unified_full_viewing_key;
pub use self::unified_full_viewing_key::*;

mod unified_spending_key;
pub use self::unified_spending_key::*;

// #[derive(Debug, PartialEq, Eq)]
pub enum ZcashDecodingError {
    // ReadError(&'static str),
    // EraInvalid,
    EraMismatch(self::era::ZcashKeysEra), // TypecodeInvalid,
                                          // LengthInvalid,
                                          // LengthMismatch(Typecode, u32),
                                          // InsufficientData(Typecode),
                                          // KeyDataInvalid(Typecode),
}
