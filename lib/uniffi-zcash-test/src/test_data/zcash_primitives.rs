use std::{io::Write, vec};

use group::GroupEncoding;
use hdwallet::ExtendedPrivKey;
use zcash_client_backend::{encoding, keys::UnifiedSpendingKey};
use zcash_primitives::{
    consensus::{MainNetwork, Parameters},
    legacy::keys::{AccountPrivKey, IncomingViewingKey},
    memo::MemoBytes,
    sapling::{
        keys::{ExpandedSpendingKey, FullViewingKey},
        Diversifier,
    },
    zip32::{ChildIndex, DiversifierIndex, ExtendedSpendingKey, Scope},
};

use super::format_bytes;

#[rustfmt::skip]
pub fn write_for_zcash_primitives<W: Write>(mut file: W, seed: &[u8]) {
    let apk = AccountPrivKey::from_seed(&MainNetwork, seed, 0.into()).unwrap();
    writeln!(file, "{}", format_bytes("account_private_key", &apk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("apk_derive_external_secret_key", &apk.derive_external_secret_key(0).unwrap().serialize_secret())).unwrap();
    writeln!(file, "{}", format_bytes("apk_derive_internal_secret_key", &apk.derive_internal_secret_key(0).unwrap().serialize_secret())).unwrap();

    let ppk = apk.to_account_pubkey();
    writeln!(file, "{}", format_bytes("account_public_key", &ppk.serialize())).unwrap();

    let external_ivk = ppk.derive_external_ivk().unwrap();
    writeln!(file, "{}", format_bytes("ppk_external_ivk", &external_ivk.serialize())).unwrap();

    let (default_address_address, default_address_index) = external_ivk.default_address();
    writeln!(file, "external_ivk_default_address_address:{}", encoding::encode_transparent_address_p(&MainNetwork, &default_address_address)).unwrap();
    writeln!(file, "external_ivk_default_address_index:{}", default_address_index).unwrap();

    let internal_ivk = ppk.derive_internal_ivk().unwrap();
    writeln!(file, "{}", format_bytes("ppk_internal_ivk", &internal_ivk.serialize())).unwrap();

    let (default_address_address, default_address_index) = internal_ivk.default_address();
    writeln!(file, "internal_ivk_default_address_address:{}", encoding::encode_transparent_address_p(&MainNetwork, &default_address_address)).unwrap();
    writeln!(file, "internal_ivk_default_address_index:{}", default_address_index).unwrap();

    writeln!(file, "{}", format_bytes("ppk_external_ovk", &ppk.external_ovk().as_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("ppk_internal_ovk", &ppk.internal_ovk().as_bytes())).unwrap();

    let extended_priv_key = ExtendedPrivKey::with_seed(seed).unwrap();
    let extended_private_key = AccountPrivKey::from_extended_privkey(extended_priv_key);
    writeln!(file, "{}", format_bytes("extended_private_key", &extended_private_key.to_bytes())).unwrap();

    writeln!(file, "t_address_public_key:tm9iMLAuYMzJ6jtFLcA7rzUmfreGuKvr7Ma").unwrap();
    writeln!(file, "t_address_script:t26YoyZ1iPgiMEWL4zGUm74eVWfhyDMXzY2").unwrap();

    let memo_bytes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    writeln!(file, "{}", format_bytes("memo_bytes", &memo_bytes)).unwrap();

    let memo_bytes = MemoBytes::from_bytes(&memo_bytes).unwrap();
    writeln!(file, "{}", format_bytes("memo_data", memo_bytes.as_slice())).unwrap();

    let memo_bytes_too_long = [0; 520];
    writeln!(file, "{}", format_bytes("memo_bytes_too_long", &memo_bytes_too_long)).unwrap();

    let memo_empty = MemoBytes::empty();
    writeln!(file, "{}", format_bytes("memo_empty", memo_empty.as_slice())).unwrap();

    let extended_spending_key = ExtendedSpendingKey::master(seed);
    let diversifiable_fvk = extended_spending_key.to_diversifiable_full_viewing_key();
    let diversifiable_fvk_fvk = diversifiable_fvk.fvk().to_bytes();
    writeln!(file, "{}", format_bytes("diversifiable_fvk_fvk", &diversifiable_fvk_fvk)).unwrap();

    let diversifiable_fvk_nk = diversifiable_fvk.to_nk(Scope::External).0.to_bytes();
    writeln!(file, "{}", format_bytes("diversifiable_fvk_nk", &diversifiable_fvk_nk)).unwrap();

    let diversifiable_fvk_ivk = diversifiable_fvk.to_ivk(Scope::External).to_repr();
    writeln!(file, "{}", format_bytes("diversifiable_fvk_ivk", &diversifiable_fvk_ivk)).unwrap();

    let diversifiable_fvk_ovk = diversifiable_fvk.to_ovk(Scope::External).0;
    writeln!(file, "{}", format_bytes("diversifiable_fvk_ovk", &diversifiable_fvk_ovk)).unwrap();

    let diversifiable_fvk_address = encoding::encode_payment_address_p(&MainNetwork, &diversifiable_fvk.address(1u32.into()).unwrap());
    writeln!(file, "diversifiable_fvk_address:{}", &diversifiable_fvk_address).unwrap();

    let (dfvk_find_address_index, dfvk_find_address_address)  = diversifiable_fvk.find_address(1u32.into()).unwrap();
    writeln!(file, "{}", format_bytes("dfvk_find_address_index", &dfvk_find_address_index.0)).unwrap();

    let dfvk_find_address_address = encoding::encode_payment_address_p(&MainNetwork, &dfvk_find_address_address);
    writeln!(file, "dfvk_find_address_address:{}", &dfvk_find_address_address).unwrap();

    let (dfvk_default_address_index, dfvk_default_address_address)  = diversifiable_fvk.default_address();
    writeln!(file, "{}", format_bytes("dfvk_default_address_index", &dfvk_default_address_index.0)).unwrap();

    let dfvk_default_address_address = encoding::encode_payment_address_p(&MainNetwork, &dfvk_default_address_address);
    writeln!(file, "dfvk_default_address_address:{}", &dfvk_default_address_address).unwrap();

    let diversifier = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let dfvk_diversified_address = diversifiable_fvk.diversified_address(Diversifier(diversifier)).unwrap();
    let dfvk_diversified_address = encoding::encode_payment_address_p(&MainNetwork, &dfvk_diversified_address);
    writeln!(file, "dfvk_diversified_address:{}", &dfvk_diversified_address).unwrap();

    let (dfvk_change_address_index, dfvk_change_address_address) = diversifiable_fvk.change_address();
    writeln!(file, "{}", format_bytes("dfvk_change_address_index", &dfvk_change_address_index.0)).unwrap();

    let dfvk_change_address_address = encoding::encode_payment_address_p(&MainNetwork, &dfvk_change_address_address);
    writeln!(file, "dfvk_change_address_address:{}", &dfvk_change_address_address).unwrap();

    let dfvk_diversified_change_address = diversifiable_fvk.diversified_change_address(Diversifier(diversifier)).unwrap();
    let dfvk_diversified_change_address = encoding::encode_payment_address_p(&MainNetwork, &dfvk_diversified_change_address);
    writeln!(file, "dfvk_diversified_change_address:{}", &dfvk_diversified_change_address).unwrap();

    let address = diversifiable_fvk.default_address().1;
    let (dfvk_decrypt_diversifier, scope) = diversifiable_fvk.decrypt_diversifier(&address).unwrap();
    writeln!(file, "{}", format_bytes("dfvk_decrypt_diversifier", &dfvk_decrypt_diversifier.0)).unwrap();
    assert_eq!(scope, Scope::External);

    let expanded_sk = ExpandedSpendingKey::from_spending_key(&extended_spending_key.to_bytes());
    writeln!(file, "{}", format_bytes("expanded_spending_key", &expanded_sk.to_bytes())).unwrap();

    #[allow(deprecated)]
    let sapling_fvk = extended_spending_key.to_extended_full_viewing_key();

    let mut sapling_fvk_bytes = vec![];
    sapling_fvk.write(&mut sapling_fvk_bytes).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk", &sapling_fvk_bytes)).unwrap();

    let sapling_fvk_encoded = encoding::encode_extended_full_viewing_key(
        MainNetwork.hrp_sapling_extended_full_viewing_key(),
        &sapling_fvk,
    );
    writeln!(file, "extended_fvk_encoded:{sapling_fvk_encoded}").unwrap();

    let child = sapling_fvk.derive_child(ChildIndex::from_index(32)).unwrap();
    let mut child_bytes = vec![];
    child.write(&mut child_bytes).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_child", &child_bytes)).unwrap();

    let address = sapling_fvk.address(4u32.into()).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_address", &address.to_bytes())).unwrap();

    let (index, address) = sapling_fvk.find_address(0u32.into()).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_find_address_index", &index.0)).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_find_address_address", &address.to_bytes())).unwrap();

    let (index, address) = sapling_fvk.default_address();
    writeln!(file, "{}", format_bytes("extended_fvk_default_address_index", &index.0)).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_default_address_address", &address.to_bytes())).unwrap();

    let mut derived_bytes = vec![];
    sapling_fvk.derive_internal().write(&mut derived_bytes).unwrap();
    writeln!(file, "{}", format_bytes("extended_fvk_derive_internal", &derived_bytes)).unwrap();

    writeln!(file, "{}", format_bytes("extended_fvk_diversifiable_fvk", &sapling_fvk.to_diversifiable_full_viewing_key().to_bytes())).unwrap();

    let sapling_fvk_from_expsk = FullViewingKey::from_expanded_spending_key(&expanded_sk);
    writeln!(file, "{}", format_bytes("sapling_full_viewing_key", &sapling_fvk_from_expsk.to_bytes())).unwrap();
    writeln!(file, "{}", format_bytes("sapling_full_viewing_key_ovk", &sapling_fvk_from_expsk.ovk.0)).unwrap();

    let usk = UnifiedSpendingKey::from_seed(&MainNetwork, seed, 0.into()).unwrap();
    let ovk = usk.to_unified_full_viewing_key()
        .sapling()
        .unwrap()
        .to_ovk(zcash_primitives::zip32::Scope::External)
        .0;
    writeln!(file, "{}", format_bytes("sapling_outgoing_viewing_key", &ovk)).unwrap();

    let vk = expanded_sk.proof_generation_key().to_viewing_key();
    let ivk = vk.ivk();
    writeln!(file, "{}", format_bytes("viewing_key_ivk", &ivk.to_repr())).unwrap();

    let address = vk.to_payment_address(Diversifier(diversifier)).unwrap();
    writeln!(file, "{}", format_bytes("viewing_key_payment_address", &address.to_bytes())).unwrap();

    let address = ivk.to_payment_address(Diversifier(diversifier)).unwrap();
    writeln!(file, "{}", format_bytes("sapling_ivk_payment_address", &address.to_bytes())).unwrap();

    let esk = ExtendedSpendingKey::master(seed);
    writeln!(file, "{}", format_bytes("extended_spending_key", &esk.to_bytes())).unwrap();

    let esk_from_path = ExtendedSpendingKey::from_path(&esk, &[ChildIndex::NonHardened(0)]);
    writeln!(file, "{}", format_bytes("esk_from_path", &esk_from_path.to_bytes())).unwrap();

    let encoded = encoding::encode_extended_spending_key(MainNetwork.hrp_sapling_extended_spending_key(), &esk);
    writeln!(file, "esk_encoded:{encoded}").unwrap();

    let child = esk.derive_child(ChildIndex::NonHardened(0));
    writeln!(file, "{}", format_bytes("extended_spending_key_child", &child.to_bytes())).unwrap();

    let (index, address) = esk.default_address();
    writeln!(file, "{}", format_bytes("esk_default_address_index", &index.0)).unwrap();
    writeln!(file, "{}", format_bytes("esk_default_address_address", &address.to_bytes())).unwrap();

    let dfvk = esk.to_diversifiable_full_viewing_key();
    writeln!(file, "{}", format_bytes("esk_to_dfvk", &dfvk.to_bytes())).unwrap();

    let mut index: DiversifierIndex = 0u32.into();
    writeln!(file, "{}", format_bytes("diversifier_index", &index.0)).unwrap();

    index.increment().unwrap();
    writeln!(file, "{}", format_bytes("diversifier_index_incremented", &index.0)).unwrap();
}
