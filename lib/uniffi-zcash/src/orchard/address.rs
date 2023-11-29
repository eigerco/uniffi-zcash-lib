use std::sync::Arc;

use orchard::Address;

use crate::{utils, ZcashError, ZcashOrchardDiversifier, ZcashResult};

use derive_more::{From, Into};

/// A shielded payment address.
#[derive(From, Into)]
pub struct ZcashOrchardAddress(pub Address);

impl ZcashOrchardAddress {
    /// Parse an address from its "raw" encoding as specified in [Zcash Protocol Spec § 5.6.4.2: Orchard Raw Payment Addresses][orchardpaymentaddrencoding]
    ///
    /// [orchardpaymentaddrencoding]: https://zips.z.cash/protocol/protocol.pdf#orchardpaymentaddrencoding
    pub fn from_raw_address_bytes(bytes: Vec<u8>) -> ZcashResult<Self> {
        let array = utils::cast_slice(&bytes)?;
        let address: Option<_> = Address::from_raw_address_bytes(&array).into();
        Ok(ZcashOrchardAddress(address.ok_or::<ZcashError>(
            "unable to parse address".to_string().into(),
        )?))
    }

    /// Returns the [`Diversifier`] for this `Address`.
    pub fn diversifier(&self) -> Arc<ZcashOrchardDiversifier> {
        Arc::new(self.0.diversifier().into())
    }

    /// Serializes this address to its "raw" encoding as specified in [Zcash Protocol Spec § 5.6.4.2: Orchard Raw Payment Addresses][orchardpaymentaddrencoding]
    ///
    /// [orchardpaymentaddrencoding]: https://zips.z.cash/protocol/protocol.pdf#orchardpaymentaddrencoding
    pub fn to_raw_address_bytes(&self) -> Vec<u8> {
        self.0.to_raw_address_bytes().to_vec()
    }
}

impl Clone for ZcashOrchardAddress {
    fn clone(&self) -> Self {
        let bs = (*self).to_raw_address_bytes().clone();

        Self::from_raw_address_bytes(bs).unwrap()
    }
}
