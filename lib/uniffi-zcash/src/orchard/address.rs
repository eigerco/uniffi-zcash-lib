use orchard::Address;

/// A shielded payment address.
pub struct ZcashOrchardAddress(pub Address);

impl ZcashOrchardAddress {
    pub fn to_raw_address_bytes(&self) -> Vec<u8> {
        self.0.to_raw_address_bytes().to_vec()
    }
}

impl From<&ZcashOrchardAddress> for Address {
    fn from(address: &ZcashOrchardAddress) -> Self {
        address.0
    }
}

impl From<Address> for ZcashOrchardAddress {
    fn from(address: Address) -> Self {
        Self(address)
    }
}
