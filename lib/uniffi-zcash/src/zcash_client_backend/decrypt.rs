use std::collections::HashMap;
use std::sync::Arc;

use zcash_client_backend::keys::UnifiedFullViewingKey;
use zcash_client_backend::{DecryptedOutput, TransferType};
use zcash_primitives::sapling;
use zcash_primitives::zip32::AccountId;

use crate::{
    ZcashAccountId, ZcashBlockHeight, ZcashConsensusParameters, ZcashMemoBytes, ZcashSaplingNote,
    ZcashTransaction, ZcashUnifiedFullViewingKey,
};

use derive_more::{From, Into};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ZcashTransferType {
    /// The output was received on one of the wallet's external addresses via decryption using the
    /// associated incoming viewing key, or at one of the wallet's transparent addresses.
    Incoming,
    /// The output was received on one of the wallet's internal-only shielded addresses via trial
    /// decryption using one of the wallet's internal incoming viewing keys.
    WalletInternal,
    /// The output was decrypted using one of the wallet's outgoing viewing keys, or was created
    /// in a transaction constructed by this wallet.
    Outgoing,
}

impl From<ZcashTransferType> for TransferType {
    fn from(value: ZcashTransferType) -> Self {
        match value {
            ZcashTransferType::Incoming => Self::Incoming,
            ZcashTransferType::WalletInternal => Self::WalletInternal,
            ZcashTransferType::Outgoing => Self::Outgoing,
        }
    }
}

impl From<TransferType> for ZcashTransferType {
    fn from(value: TransferType) -> Self {
        match value {
            TransferType::Incoming => Self::Incoming,
            TransferType::WalletInternal => Self::WalletInternal,
            TransferType::Outgoing => Self::Outgoing,
        }
    }
}

/// A decrypted shielded output.
#[derive(From, Into)]
pub struct ZcashDecryptedOutput(DecryptedOutput<sapling::Note>);

impl ZcashDecryptedOutput {
    pub fn index(&self) -> u64 {
        self.0.index.try_into().unwrap()
    }

    pub fn note(&self) -> Arc<ZcashSaplingNote> {
        Arc::new(self.0.note.clone().into())
    }

    pub fn account(&self) -> ZcashAccountId {
        ZcashAccountId {
            id: self.0.account.into(),
        }
    }

    pub fn memo(&self) -> Arc<ZcashMemoBytes> {
        Arc::new(self.0.memo.clone().into())
    }

    /// True if this output was recovered using an [`OutgoingViewingKey`], meaning that
    /// this is a logical output of the transaction.
    ///
    /// [`OutgoingViewingKey`]: zcash_primitives::keys::OutgoingViewingKey
    pub fn transfer_type(&self) -> ZcashTransferType {
        self.0.transfer_type.into()
    }
}

pub fn decrypt_transaction(
    params: ZcashConsensusParameters,
    height: Arc<ZcashBlockHeight>,
    tx: Arc<ZcashTransaction>,
    ufvks: HashMap<ZcashAccountId, Arc<ZcashUnifiedFullViewingKey>>,
) -> Vec<Arc<ZcashDecryptedOutput>> {
    let ufvks: HashMap<AccountId, UnifiedFullViewingKey> = ufvks
        .into_iter()
        .map(|(x, y)| (x.into(), (*y).clone().into()))
        .collect();

    zcash_client_backend::decrypt_transaction(
        &params,
        (*height).into(),
        &(*tx).clone().into(),
        &ufvks,
    )
    .into_iter()
    .map(From::from)
    .map(Arc::new)
    .collect()
}
