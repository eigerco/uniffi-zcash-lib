use delegate::delegate;
use std::sync::Arc;
use zcash_primitives::consensus::{MainNetwork, TestNetwork};

mod utils;

mod unified_address;
mod sapling;
mod transparent;

#[cfg(feature = "rustler")]
mod beam;

pub use self::unified_address::*;
pub use self::sapling::*;
pub use self::transparent::*;

/// Zcash error.
#[derive(Debug, thiserror::Error)]
pub enum ZcashError {
    #[error("hdwallet error occurred: {error:?}")]
    HDWalletError { error: hdwallet::error::Error },

    #[error("derivation error occurred: {error:?}")]
    DerivationError {
        error: zcash_client_backend::keys::DerivationError,
    },

    #[error("error occurred: {error}")]
    Message { error: String },

    #[error("expected {expected} elements, got {got}")]
    ArrayLengthMismatch { expected: u64, got: u64 },

    #[error("unknown error occurred")]
    Unknown,
}

type ZcashResult<T> = Result<T, ZcashError>;

impl From<hdwallet::error::Error> for ZcashError {
    fn from(error: hdwallet::error::Error) -> Self {
        ZcashError::HDWalletError { error }
    }
}

impl From<zcash_client_backend::keys::DerivationError> for ZcashError {
    fn from(error: zcash_client_backend::keys::DerivationError) -> Self {
        ZcashError::DerivationError { error }
    }
}

impl From<String> for ZcashError {
    fn from(error: String) -> Self {
        ZcashError::Message { error }
    }
}

/// Zcash consensus parameters.
#[derive(Copy, Clone)]
pub enum ZcashConsensusParameters {
    /// Marker struct for the production network.
    MainNetwork,

    /// Marker struct for the test network.
    TestNetwork,
    // todo: expose a way to use other types of parameters
}

impl zcash_primitives::consensus::Parameters for ZcashConsensusParameters {
    delegate! {
        to match self {
            ZcashConsensusParameters::MainNetwork => MainNetwork,
            ZcashConsensusParameters::TestNetwork => TestNetwork,
        } {
            fn activation_height(
                &self,
                nu: zcash_primitives::consensus::NetworkUpgrade,
            ) -> Option<zcash_primitives::consensus::BlockHeight>;

            fn coin_type(&self) -> u32;

            fn address_network(&self) -> Option<zcash_address::Network>;

            fn hrp_sapling_extended_spending_key(&self) -> &str;

            fn hrp_sapling_extended_full_viewing_key(&self) -> &str;

            fn hrp_sapling_payment_address(&self) -> &str;

            fn b58_pubkey_address_prefix(&self) -> [u8; 2];

            fn b58_script_address_prefix(&self) -> [u8; 2];
        }
    }
}

/// A type-safe wrapper for account identifiers.
#[derive(Copy, Clone)]
pub struct ZcashAccountId {
    pub id: u32,
}

impl From<ZcashAccountId> for zcash_primitives::zip32::AccountId {
    fn from(value: ZcashAccountId) -> Self {
        value.id.into()
    }
}

/// A type representing a BIP-44 private key at the account path level
/// `m/44'/<coin_type>'/<account>'
pub struct ZcashAccountPrivKey {
    inner: zcash_primitives::legacy::keys::AccountPrivKey,
}

impl From<zcash_primitives::legacy::keys::AccountPrivKey> for ZcashAccountPrivKey {
    fn from(inner: zcash_primitives::legacy::keys::AccountPrivKey) -> Self {
        ZcashAccountPrivKey { inner }
    }
}

impl ZcashAccountPrivKey {
    /// Performs derivation of the extended private key for the BIP-44 path:
    /// `m/44'/<coin_type>'/<account>'`.
    ///
    /// This produces the root of the derivation tree for transparent
    /// viewing keys and addresses for the for the provided account.
    pub fn from_seed(
        params: ZcashConsensusParameters,
        seed: Vec<u8>,
        account: ZcashAccountId,
    ) -> ZcashResult<Self> {
        let key = zcash_primitives::legacy::keys::AccountPrivKey::from_seed(
            &params,
            &seed,
            account.into(),
        )?;

        Ok(key.into())
    }

    pub fn from_extended_privkey(ext_privkey: Arc<ZcashExtendedPrivKey>) -> Self {
        let key = zcash_primitives::legacy::keys::AccountPrivKey::from_extended_privkey(
            ext_privkey.inner.clone(),
        );
        key.into()
    }

    pub fn to_account_pubkey(&self) -> Arc<ZcashAccountPubKey> {
        Arc::new(self.inner.to_account_pubkey().into())
    }

    /*
    /// Derives the BIP-44 private spending key for the external (incoming payment) child path
    /// `m/44'/<coin_type>'/<account>'/0/<child_index>`.
    pub fn derive_external_secret_key(
        &self,
        child_index: u32,
    ) -> Result<secp256k1::SecretKey, hdwallet::error::Error> {
        todo!()
    }

    /// Derives the BIP-44 private spending key for the internal (change) child path
    /// `m/44'/<coin_type>'/<account>'/1/<child_index>`.
    pub fn derive_internal_secret_key(
        &self,
        child_index: u32,
    ) -> Result<secp256k1::SecretKey, hdwallet::error::Error> {
        todo!()
    }
    */

    /// Returns the `AccountPrivKey` serialized using the encoding for a
    /// [BIP 32](https://en.bitcoin.it/wiki/BIP_0032) ExtendedPrivKey
    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.to_bytes()
    }

    /// Decodes the `AccountPrivKey` from the encoding specified for a
    /// [BIP 32](https://en.bitcoin.it/wiki/BIP_0032) ExtendedPrivKey
    pub fn from_bytes(bytes: Vec<u8>) -> ZcashResult<Self> {
        zcash_primitives::legacy::keys::AccountPrivKey::from_bytes(&bytes)
            .map(ZcashAccountPrivKey::from)
            .ok_or(ZcashError::Unknown)
    }
}

/// ExtendedPrivKey is used for child key derivation.
/// See [secp256k1 crate documentation](https://docs.rs/secp256k1) for SecretKey signatures usage.
pub struct ZcashExtendedPrivKey {
    inner: hdwallet::extended_key::ExtendedPrivKey,
}

impl ZcashExtendedPrivKey {
    fn with_seed(seed: Vec<u8>) -> ZcashResult<Self> {
        let key = hdwallet::extended_key::ExtendedPrivKey::with_seed(&seed)?;

        Ok(key.into())
    }
}

impl From<hdwallet::extended_key::ExtendedPrivKey> for ZcashExtendedPrivKey {
    fn from(inner: hdwallet::extended_key::ExtendedPrivKey) -> Self {
        Self { inner }
    }
}

/// A type representing a BIP-44 public key at the account path level
/// `m/44'/<coin_type>'/<account>'`.
///
/// This provides the necessary derivation capability for the transparent component of a unified
/// full viewing key.
pub struct ZcashAccountPubKey {
    inner: zcash_primitives::legacy::keys::AccountPubKey,
}

impl From<zcash_primitives::legacy::keys::AccountPubKey> for ZcashAccountPubKey {
    fn from(inner: zcash_primitives::legacy::keys::AccountPubKey) -> Self {
        ZcashAccountPubKey { inner }
    }
}

/// A Sapling extended spending key
pub struct ZcashExtendedSpendingKey {
    inner: zcash_primitives::zip32::sapling::ExtendedSpendingKey,
}

impl From<zcash_primitives::zip32::sapling::ExtendedSpendingKey> for ZcashExtendedSpendingKey {
    fn from(inner: zcash_primitives::zip32::sapling::ExtendedSpendingKey) -> Self {
        ZcashExtendedSpendingKey { inner }
    }
}

impl ZcashExtendedSpendingKey {
    pub fn master(seed: &[u8]) -> Self {
        todo!()
    }

    /// Decodes the extended spending key from its serialized representation as defined in
    /// [ZIP 32](https://zips.z.cash/zip-0032)
    pub fn from_bytes(bytes: Vec<u8>) -> ZcashResult<Self> {
        todo!()
    }

    /// Encodes the extended spending key to the its seralized representation as defined in
    /// [ZIP 32](https://zips.z.cash/zip-0032)
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    /*
    /// Returns the child key corresponding to the path derived from the master key
    pub fn from_path(master: &ExtendedSpendingKey, path: &[ChildIndex]) -> Self {
    }

    #[must_use]
    pub fn derive_child(&self, i: ChildIndex) -> Self {
    }

    /// Returns the address with the lowest valid diversifier index, along with
    /// the diversifier index that generated that address.
    pub fn default_address(&self) -> (DiversifierIndex, PaymentAddress) {
    }
    */

    /// Derives an internal spending key given an external spending key.
    ///
    /// Specified in [ZIP 32](https://zips.z.cash/zip-0032#deriving-a-sapling-internal-spending-key).
    #[must_use]
    pub fn derive_internal(&self) -> Self {
        todo!()
    }

    /*
    pub fn to_diversifiable_full_viewing_key(&self) -> DiversifiableFullViewingKey {
        todo!()
    }
    */
}

/// A version identifier for the encoding of unified spending keys.
///
/// Each era corresponds to a range of block heights. During an era, the unified spending key
/// parsed from an encoded form tagged with that era's identifier is expected to provide
/// sufficient spending authority to spend any non-Sprout shielded note created in a transaction
/// within the era's block range.
pub enum ZcashKeysEra {
    /// The Orchard era begins at Orchard activation, and will end if a new pool that requires a
    /// change to unified spending keys is introduced.
    Orchard,
}

impl From<ZcashKeysEra> for zcash_client_backend::keys::Era {
    fn from(value: ZcashKeysEra) -> Self {
        match value {
            ZcashKeysEra::Orchard => zcash_client_backend::keys::Era::Orchard,
        }
    }
}

/// A set of viewing keys that are all associated with a single
/// ZIP-0032 account identifier.
pub struct ZcashUnifiedSpendingKey {
    inner: zcash_client_backend::keys::UnifiedSpendingKey,
}

impl From<zcash_client_backend::keys::UnifiedSpendingKey> for ZcashUnifiedSpendingKey {
    fn from(inner: zcash_client_backend::keys::UnifiedSpendingKey) -> Self {
        ZcashUnifiedSpendingKey { inner }
    }
}

impl ZcashUnifiedSpendingKey {
    pub fn from_seed(
        params: ZcashConsensusParameters,
        seed: Vec<u8>,
        account: ZcashAccountId,
    ) -> ZcashResult<Self> {
        let key = zcash_client_backend::keys::UnifiedSpendingKey::from_seed(
            &params,
            &seed,
            account.into(),
        )?;

        Ok(key.into())
    }

    pub fn to_unified_full_viewing_key(&self) -> Arc<ZcashUnifiedFullViewingKey> {
        Arc::new(self.inner.to_unified_full_viewing_key().into())
    }

    /// Returns a binary encoding of this key suitable for decoding with [`decode`].
    ///
    /// The encoded form of a unified spending key is only intended for use
    /// within wallets when required for storage and/or crossing FFI boundaries;
    /// unified spending keys should not be exposed to users, and consequently
    /// no string-based encoding is defined. This encoding does not include any
    /// internal validation metadata (such as checksums) as keys decoded from
    /// this form will necessarily be validated when the attempt is made to
    /// spend a note that they have authority for.
    pub fn to_bytes(&self, era: ZcashKeysEra) -> Vec<u8> {
        self.inner.to_bytes(era.into())
    }
}

/// A [ZIP 316](https://zips.z.cash/zip-0316) unified full viewing key.
pub struct ZcashUnifiedFullViewingKey {
    inner: zcash_client_backend::keys::UnifiedFullViewingKey,
}

impl From<zcash_client_backend::keys::UnifiedFullViewingKey> for ZcashUnifiedFullViewingKey {
    fn from(inner: zcash_client_backend::keys::UnifiedFullViewingKey) -> Self {
        ZcashUnifiedFullViewingKey { inner }
    }
}

impl ZcashUnifiedFullViewingKey {
    /// Parses a `UnifiedFullViewingKey` from its [ZIP 316] string encoding.
    ///
    /// [ZIP 316]: https://zips.z.cash/zip-0316
    pub fn decode(params: ZcashConsensusParameters, encoding: &str) -> ZcashResult<Self> {
        let key = zcash_client_backend::keys::UnifiedFullViewingKey::decode(&params, encoding)?;

        Ok(key.into())
    }

    /// Returns the string encoding of this `UnifiedFullViewingKey` for the given network.
    pub fn encode(&self, params: ZcashConsensusParameters) -> String {
        self.inner.encode(&params)
    }

    /// Returns the Sapling diversifiable full viewing key component of this unified key.
    pub fn sapling(&self) -> Option<Arc<ZcashDiversifiableFullViewingKey>> {
        self.inner
            .sapling()
            .cloned()
            .map(ZcashDiversifiableFullViewingKey::from)
            .map(Arc::new)
    }

    pub fn orchard(&self) -> Option<Arc<ZcashOrchardFullViewingKey>> {
        self.inner
            .orchard()
            .cloned()
            .map(ZcashOrchardFullViewingKey::from)
            .map(Arc::new)
    }
}

/// A Sapling key that provides the capability to view incoming and outgoing transactions.
///
/// This key is useful anywhere you need to maintain accurate balance, but do not want the
/// ability to spend funds (such as a view-only wallet).
///
/// It comprises the subset of the ZIP 32 extended full viewing key that is used for the
/// Sapling item in a [ZIP 316 Unified Full Viewing Key][zip-0316-ufvk].
///
/// [zip-0316-ufvk]: https://zips.z.cash/zip-0316#encoding-of-unified-full-incoming-viewing-keys
pub struct ZcashDiversifiableFullViewingKey {
    inner: zcash_client_backend::keys::sapling::DiversifiableFullViewingKey,
}

impl From<zcash_client_backend::keys::sapling::DiversifiableFullViewingKey>
    for ZcashDiversifiableFullViewingKey
{
    fn from(inner: zcash_client_backend::keys::sapling::DiversifiableFullViewingKey) -> Self {
        ZcashDiversifiableFullViewingKey { inner }
    }
}

impl ZcashDiversifiableFullViewingKey {
    /// Parses a `DiversifiableFullViewingKey` from its raw byte encoding.
    ///
    /// Returns `None` if the bytes do not contain a valid encoding of a diversifiable
    /// Sapling full viewing key.
    pub fn from_bytes(bytes: Vec<u8>) -> ZcashResult<Self> {
        let array = utils::cast_slice(&bytes)?;
        let key =
            zcash_client_backend::keys::sapling::DiversifiableFullViewingKey::from_bytes(&array)
                .ok_or(ZcashError::Unknown)?;

        Ok(ZcashDiversifiableFullViewingKey { inner: key })
    }
    /// Returns the raw encoding of this `DiversifiableFullViewingKey`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.to_bytes().into()
    }

    /*
    /// Exposes the external [`FullViewingKey`] component of this diversifiable full viewing key.
    pub fn fvk(&self) -> &FullViewingKey {
        &self.fvk
    }

    /// Derives a nullifier-deriving key for the provided scope.
    ///
    /// This API is provided so that nullifiers for change notes can be correctly computed.
    pub fn to_nk(&self, scope: Scope) -> NullifierDerivingKey {
        match scope {
            Scope::External => self.fvk.vk.nk,
            Scope::Internal => self.derive_internal().fvk.vk.nk,
        }
    }
    */

    /// Derives an incoming viewing key corresponding to this full viewing key.
    pub fn to_ivk(&self, scope: ZcashScope) -> Arc<ZcashSaplingIvk> {
        Arc::new(self.inner.to_ivk(scope.into()).into())
    }

    /// Derives an outgoing viewing key corresponding to this full viewing key.
    pub fn to_ovk(&self, scope: ZcashScope) -> Arc<ZcashOutgoingViewingKey> {
        Arc::new(self.inner.to_ovk(scope.into()).into())
    }

    /*
    /// Attempts to produce a valid payment address for the given diversifier index.
    ///
    /// Returns `None` if the diversifier index does not produce a valid diversifier for
    /// this `DiversifiableFullViewingKey`.
    pub fn address(&self, j: DiversifierIndex) -> Option<PaymentAddress> {
        sapling_address(&self.fvk, &self.dk, j)
    }

    /// Finds the next valid payment address starting from the given diversifier index.
    ///
    /// This searches the diversifier space starting at `j` and incrementing, to find an
    /// index which will produce a valid diversifier (a 50% probability for each index).
    ///
    /// Returns the index at which the valid diversifier was found along with the payment
    /// address constructed using that diversifier, or `None` if the maximum index was
    /// reached and no valid diversifier was found.
    pub fn find_address(&self, j: DiversifierIndex) -> Option<(DiversifierIndex, PaymentAddress)> {
        sapling_find_address(&self.fvk, &self.dk, j)
    }

    /// Returns the payment address corresponding to the smallest valid diversifier index,
    /// along with that index.
    pub fn default_address(&self) -> (DiversifierIndex, PaymentAddress) {
        sapling_default_address(&self.fvk, &self.dk)
    }

    /// Returns the payment address corresponding to the specified diversifier, if any.
    ///
    /// In general, it is preferable to use `find_address` instead, but this method is
    /// useful in some cases for matching keys to existing payment addresses.
    pub fn diversified_address(&self, diversifier: Diversifier) -> Option<PaymentAddress> {
        self.fvk.vk.to_payment_address(diversifier)
    }

    /// Returns the internal address corresponding to the smallest valid diversifier index,
    /// along with that index.
    ///
    /// This address **MUST NOT** be encoded and exposed to end users. User interfaces
    /// should instead mark these notes as "change notes" or "internal wallet operations".
    pub fn change_address(&self) -> (DiversifierIndex, PaymentAddress) {
        let internal_dfvk = self.derive_internal();
        sapling_default_address(&internal_dfvk.fvk, &internal_dfvk.dk)
    }

    /// Returns the change address corresponding to the specified diversifier, if any.
    ///
    /// In general, it is preferable to use `change_address` instead, but this method is
    /// useful in some cases for matching keys to existing payment addresses.
    pub fn diversified_change_address(&self, diversifier: Diversifier) -> Option<PaymentAddress> {
        self.derive_internal()
            .fvk
            .vk
            .to_payment_address(diversifier)
    }

    /// Attempts to decrypt the given address's diversifier with this full viewing key.
    ///
    /// This method extracts the diversifier from the given address and decrypts it as a
    /// diversifier index, then verifies that this diversifier index produces the same
    /// address. Decryption is attempted using both the internal and external parts of the
    /// full viewing key.
    ///
    /// Returns the decrypted diversifier index and its scope, or `None` if the address
    /// was not generated from this key.
    pub fn decrypt_diversifier(&self, addr: &PaymentAddress) -> Option<(DiversifierIndex, Scope)> {
        let j_external = self.dk.diversifier_index(addr.diversifier());
        if self.address(j_external).as_ref() == Some(addr) {
            return Some((j_external, Scope::External));
        }

        let j_internal = self
            .derive_internal()
            .dk
            .diversifier_index(addr.diversifier());
        if self.address(j_internal).as_ref() == Some(addr) {
            return Some((j_internal, Scope::Internal));
        }

        None
    } */
}

pub struct ZcashSaplingIvk {
    inner: zcash_primitives::sapling::SaplingIvk,
}

impl From<zcash_primitives::sapling::SaplingIvk> for ZcashSaplingIvk {
    fn from(inner: zcash_primitives::sapling::SaplingIvk) -> Self {
        ZcashSaplingIvk { inner }
    }
}

impl ZcashSaplingIvk {
    pub fn to_payment_address(
        &self,
        diversifier: Arc<ZcashDiversifier>,
    ) -> Option<Arc<ZcashPaymentAddress>> {
        self.inner
            .to_payment_address(diversifier.as_ref().into())
            .map(ZcashPaymentAddress::from)
            .map(Arc::new)
    }

    pub fn to_repr(&self) -> Vec<u8> {
        self.inner.to_repr().into()
    }
}

/// An outgoing viewing key
pub struct ZcashOutgoingViewingKey {
    inner: zcash_primitives::sapling::keys::OutgoingViewingKey,
}

impl ZcashOutgoingViewingKey {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.0.to_vec()
    }
}

impl From<zcash_primitives::sapling::keys::OutgoingViewingKey> for ZcashOutgoingViewingKey {
    fn from(inner: zcash_primitives::sapling::keys::OutgoingViewingKey) -> Self {
        ZcashOutgoingViewingKey { inner }
    }
}

/// The scope of a viewing key or address.
///
/// A "scope" narrows the visibility or usage to a level below "full".
///
/// Consistent usage of `Scope` enables the user to provide consistent views over a wallet
/// to other people. For example, a user can give an external [SaplingIvk] to a merchant
/// terminal, enabling it to only detect "real" transactions from customers and not
/// internal transactions from the wallet.
///
/// [SaplingIvk]: ZcashSaplingIvk
pub enum ZcashScope {
    /// A scope used for wallet-external operations, namely deriving addresses to give to
    /// other users in order to receive funds.
    External,
    /// A scope used for wallet-internal operations, such as creating change notes,
    /// auto-shielding, and note management.
    Internal,
}

impl From<ZcashScope> for zcash_primitives::zip32::Scope {
    fn from(value: ZcashScope) -> Self {
        match value {
            ZcashScope::External => zcash_primitives::zip32::Scope::External,
            ZcashScope::Internal => zcash_primitives::zip32::Scope::Internal,
        }
    }
}

/// A key that provides the capability to view incoming and outgoing transactions.
///
/// This key is useful anywhere you need to maintain accurate balance, but do not want the
/// ability to spend funds (such as a view-only wallet).
pub struct ZcashOrchardFullViewingKey {
    inner: orchard::keys::FullViewingKey,
}

impl ZcashOrchardFullViewingKey {
    pub fn to_ivk(&self, scope: ZcashOrchardScope) -> Arc<ZcashOrchardIncomingViewingKey> {
        Arc::new(self.inner.to_ivk(scope.into()).into())
    }
    pub fn to_ovk(&self, scope: ZcashOrchardScope) -> Arc<ZcashOrchardOutgoingViewingKey> {
        Arc::new(self.inner.to_ovk(scope.into()).into())
    }
}

impl From<orchard::keys::FullViewingKey> for ZcashOrchardFullViewingKey {
    fn from(inner: orchard::keys::FullViewingKey) -> Self {
        Self { inner }
    }
}

/// The scope of a viewing key or address.
pub enum ZcashOrchardScope {
    /// A scope used for wallet-external operations, namely deriving addresses to give to
    /// other users in order to receive funds.
    External,
    /// A scope used for wallet-internal operations, such as creating change notes,
    /// auto-shielding, and note management.
    Internal,
}

impl From<ZcashOrchardScope> for orchard::keys::Scope {
    fn from(value: ZcashOrchardScope) -> Self {
        match value {
            ZcashOrchardScope::External => orchard::keys::Scope::External,
            ZcashOrchardScope::Internal => orchard::keys::Scope::Internal,
        }
    }
}

/// Orchard

/// A key that provides the capability to detect and decrypt incoming notes from the block
/// chain, without being able to spend the notes or detect when they are spent.
///
/// This key is useful in situations where you only need the capability to detect inbound
/// payments, such as merchant terminals.
pub struct ZcashOrchardIncomingViewingKey {
    inner: orchard::keys::IncomingViewingKey,
}

impl ZcashOrchardIncomingViewingKey {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.to_bytes().to_vec()
    }

    pub fn address(&self, diversifier: Arc<ZcashOrchardDiversifier>) -> Arc<ZcashOrchardAddress> {
        Arc::new(self.inner.address(diversifier.inner).into())
    }
}

impl From<orchard::keys::IncomingViewingKey> for ZcashOrchardIncomingViewingKey {
    fn from(inner: orchard::keys::IncomingViewingKey) -> Self {
        Self { inner }
    }
}

/// A diversifier that can be used to derive a specific [`Address`] from a
/// [`FullViewingKey`] or [`IncomingViewingKey`].
pub struct ZcashOrchardDiversifier {
    inner: orchard::keys::Diversifier,
}

impl ZcashOrchardDiversifier {
    pub fn from_bytes(data: Vec<u8>) -> ZcashResult<Self> {
        let array = utils::cast_slice(&data)?;
        Ok(orchard::keys::Diversifier::from_bytes(array).into())
    }
}

impl From<orchard::keys::Diversifier> for ZcashOrchardDiversifier {
    fn from(inner: orchard::keys::Diversifier) -> Self {
        Self { inner }
    }
}

/// A key that provides the capability to recover outgoing transaction information from
/// the block chain.
pub struct ZcashOrchardOutgoingViewingKey {
    inner: orchard::keys::OutgoingViewingKey,
}

impl From<orchard::keys::OutgoingViewingKey> for ZcashOrchardOutgoingViewingKey {
    fn from(inner: orchard::keys::OutgoingViewingKey) -> Self {
        Self { inner }
    }
}

/// A shielded payment address.
pub struct ZcashOrchardAddress {
    inner: orchard::Address,
}

impl ZcashOrchardAddress {
    pub fn to_raw_address_bytes(&self) -> Vec<u8> {
        self.inner.to_raw_address_bytes().to_vec()
    }
}

impl From<orchard::Address> for ZcashOrchardAddress {
    fn from(inner: orchard::Address) -> Self {
        Self { inner }
    }
}

uniffi::include_scaffolding!("backend_keys");
