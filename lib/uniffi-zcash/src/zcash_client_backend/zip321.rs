use std::fmt;

use zcash_client_backend::zip321::{TransactionRequest, Zip321Error};

use crate::{ZcashConsensusParameters, ZcashPayment};

#[derive(Debug)]
pub enum ZcashZip321Error {
    /// A memo field in the ZIP 321 URI was not properly base-64 encoded
    // InvalidBase64(base64::DecodeError),
    /// A memo value exceeded 512 bytes in length or could not be interpreted as a UTF-8 string
    /// when using a valid UTF-8 lead byte.
    // MemoBytesError(memo::Error),
    /// Parsing encountered a duplicate ZIP 321 URI parameter for the returned payment index.
    // DuplicateParameter{v: u32}, // parse::Param,
    /// The ZIP 321 request included more payments than can be created within a single Zcash
    /// transaction. The wrapped value is the number of payments in the request.
    TooManyPayments { v: u32 },
    /// The payment at the wrapped index attempted to include a memo when sending to a
    /// transparent recipient address, which is not supported by the protocol.
    TransparentMemo { v: u32 },
    /// The payment at the wrapped index did not include a recipient address.
    RecipientMissing { v: u32 },
    /// The ZIP 321 URI was malformed and failed to parse.
    ParseError { v: String },
}

impl fmt::Display for ZcashZip321Error {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZcashZip321Error::RecipientMissing { v } => write!(f, "{}", v),
            ZcashZip321Error::TransparentMemo { v } => write!(f, "{}", v),
            ZcashZip321Error::TooManyPayments { v } => write!(f, "{}", v),
            ZcashZip321Error::ParseError { v } => write!(f, "{}", v),
        }
    }
}

impl From<ZcashZip321Error> for Zip321Error {
    fn from(value: ZcashZip321Error) -> Self {
        match value {
            ZcashZip321Error::RecipientMissing { v } => {
                Zip321Error::RecipientMissing(v.try_into().unwrap())
            }
            ZcashZip321Error::TooManyPayments { v } => {
                Zip321Error::TooManyPayments(v.try_into().unwrap())
            }
            ZcashZip321Error::TransparentMemo { v } => {
                Zip321Error::TransparentMemo(v.try_into().unwrap())
            }
            ZcashZip321Error::ParseError { v } => match v.as_str() {
                "InvalidBase64" => Zip321Error::ParseError("InvalidBase64".to_string()),
                "MemoBytesError" => Zip321Error::ParseError("MemoBytesError".to_string()),
                "DuplicateParameter" => Zip321Error::ParseError("MemoBytesError".to_string()),
                _ => Zip321Error::ParseError(v),
            },
        }
    }
}

impl From<Zip321Error> for ZcashZip321Error {
    fn from(value: Zip321Error) -> Self {
        match value {
            Zip321Error::RecipientMissing(v) => ZcashZip321Error::RecipientMissing {
                v: v.try_into().unwrap(),
            },
            Zip321Error::TooManyPayments(v) => ZcashZip321Error::TooManyPayments {
                v: v.try_into().unwrap(),
            },
            Zip321Error::TransparentMemo(v) => ZcashZip321Error::TransparentMemo {
                v: v.try_into().unwrap(),
            },
            Zip321Error::ParseError(v) => ZcashZip321Error::ParseError { v },
            Zip321Error::InvalidBase64 { .. } => ZcashZip321Error::ParseError {
                v: "InvalidBase64".to_string(),
            },
            Zip321Error::MemoBytesError { .. } => ZcashZip321Error::ParseError {
                v: "MemoBytesError".to_string(),
            },
            Zip321Error::DuplicateParameter { .. } => ZcashZip321Error::ParseError {
                v: "DuplicateParameter".to_string(),
            },
        }
    }
}

pub type ZcashZip321Result<T> = Result<T, ZcashZip321Error>;

/// A ZIP321 transaction request.
///
/// A ZIP 321 request may include one or more such requests for payment.
/// When constructing a transaction in response to such a request,
/// a separate output should be added to the transaction for each
/// payment value in the request.
#[derive(Debug)]
pub struct ZcashTransactionRequest(TransactionRequest);

impl ZcashTransactionRequest {
    /// Constructs a new empty transaction request.
    pub fn empty() -> Self {
        TransactionRequest::empty().into()
    }

    /// Constructs a new transaction request that obeys the ZIP-321 invariants
    pub fn new(payments: Vec<ZcashPayment>) -> ZcashZip321Result<Self> {
        TransactionRequest::new(payments.into_iter().map(From::from).collect())
            .map(From::from)
            .map_err(From::from)
    }

    /// Parse the provided URI to a payment request value.
    pub fn from_uri(params: ZcashConsensusParameters, uri: &str) -> ZcashZip321Result<Self> {
        TransactionRequest::from_uri(&params, uri)
            .map_err(From::from)
            .map(From::from)
    }

    /// Returns the slice of payments that make up this request.
    pub fn payments(&self) -> Vec<ZcashPayment> {
        self.0.payments().iter().map(From::from).collect()
    }

    /// Convert this request to a URI string.
    ///
    /// Returns None if the payment request is empty.
    pub fn to_uri(&self, params: ZcashConsensusParameters) -> Option<String> {
        self.0.to_uri(&params)
    }
}


impl From<ZcashTransactionRequest> for TransactionRequest {
    fn from(inner: ZcashTransactionRequest) -> Self {
        inner.0
    }
}

impl From<TransactionRequest> for ZcashTransactionRequest {
    fn from(e: TransactionRequest) -> Self {
        ZcashTransactionRequest(e)
    }
}
