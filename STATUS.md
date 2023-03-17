
## Completion map

We currently have implemented and tested the following object graph:

<a name="completion_map"></a>

```mermaid
graph LR;
style UnifiedSK fill:#6FBD11
style UnifiedFVK fill:#6FBD11
style SaplingDiversifiableFVK fill:#6FBD11
style SaplingIVK fill:#6FBD11
style SaplingOVK fill:#6FBD11
style PaymentAddress fill:#6FBD11
style OrchardFVK fill:#6FBD11
style OrchardIVK fill:#6FBD11
style OrchardOVK fill:#6FBD11
style OrchardAddress fill:#6FBD11
style SaplingExtendedSK fill:#6FBD11
style OrchardSK fill:#6FBD11


UnifiedSK-->TransparentAccountPrivKey
TransparentAccountPrivKey-->TransparentAccountPubKey
UnifiedSK-->SaplingExtendedSK
SaplingExtendedSK-->SaplingExtendedFVK
SaplingExtendedFVK-->SaplingDiversifiableFVK
SaplingDiversifiableFVK-->SaplingIVK
SaplingIVK-->PaymentAddress
SaplingDiversifiableFVK-->SaplingOVK
SaplingExtendedSK-->SaplingDiversifiableFVK
UnifiedSK-->OrchardSK
OrchardSK-->OrchardFVK
OrchardFVK-->OrchardIVK
OrchardIVK--->OrchardAddress
OrchardFVK-->OrchardOVK
UnifiedFVK-->TransparentAccountPubKey
UnifiedFVK-->SaplingDiversifiableFVK
UnifiedFVK-->OrchardFVK
UnifiedSK-->UnifiedFVK
```
* See API surface section below to see the API detailed status.

## FFI API features matrix

This is a feature matrix which keeps track of the current state of implementation for the FFI API surface. Here are the descriptions of each column:

* `Object/Method name`: The listing of all the `pub` methods that could be implemented for FFI .
* `Score`: How important is nowadays this. Being.
  * :red_circle: Essential.
  * :large_blue_circle: Nice to have.
  * :white_circle: Not interesting.
  *  If they do not have colour, evaluation is still needed.
* `UDL`:  The code is present in the Mozilla `UDL` file.
* `Code`: If theres an existing implementation in the Rust wrapper code.
* `Test`: Theres test coverage for this element. Directly or indirectly.
* `Docs`: If there is function level documentation in the `FFI` code, which later will be automatically generated bindings for all languages ([we are working on it](https://github.com/eigerco/uniffi-rs/issues)). See format to be followed [here](https://github.com/eigerco/uniffi-rs/issues/5#issuecomment-1436618667).

**Notes** : 

* This should be kept up to date with new developments/findings. 
* Its still in progress the creation of new types for `Orchard` below in this index.

### ZcashAccountPrivKey

* Original type: [zcash_primitives::legacy::keys::AccountPrivKey](https://docs.rs/zcash_primitives/0.10.0/zcash_primitives/legacy/keys/struct.AccountPrivKey.html)

| Object/Method name                                                                          |    Score     |        UDL         |        Code        |       Tests        | Docs  |
| ------------------------------------------------------------------------------------------- | :----------: | :----------------: | :----------------: | :----------------: | :---: |
| ZcashAccountPrivKey::from_seed()                                                            | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashAccountPrivKey::from_extended_privkey([ZcashExtendedPrivKey](###ZcashExtendedPrivKey)) | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |
| ZcashAccountPrivKey::to_account_pubkey()                                                    | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |
| ZcashAccountPrivKey::to_extended_privkey()                                                  |              |                    |                    |                    |       |
| ZcashAccountPrivKey::derive_external_secret_key()                                           |              |                    |                    |                    |       |
| ZcashAccountPrivKey::derive_internal_secret_key()                                           |              |                    |                    |                    |       |
| ZcashAccountPrivKey::to_bytes()                                                             | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashAccountPrivKey::from_bytes()                                                           | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |

### ZcashExtendedPrivKey

* Original type: [hdwallet::extended_key::ExtendedPrivKey](https://docs.rs/hdwallet/latest/hdwallet/extended_key/struct.ExtendedPrivKey.html)

| Object/Method name                            |    Score     |        UDL         |        Code        |       Tests        | Docs  |
| --------------------------------------------- | :----------: | :----------------: | :----------------: | :----------------: | :---: |
| ZcashExtendedPrivKey::random()                |              |                    |                    |                    |       |
| ZcashExtendedPrivKey::random_with_seed_size() |              |                    |                    |                    |       |
| ZcashExtendedPrivKey::with_seed()             | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashExtendedPrivKey::derive_private_key()    |              |                    |                    |                    |       |

### ZcashAccountPubKey

* Original type: [zcash_primitives::legacy::keys::AccountPubKey](https://docs.rs/zcash_primitives/0.10.0/zcash_primitives/legacy/keys/struct.AccountPubKey.html)

Currently this entity is only present in the `UDL` and the `Code` , but without any method implementation.

| Object/Method name                        | Score |  UDL  | Code  | Tests | Docs  |
| ----------------------------------------- | :---: | :---: | :---: | :---: | :---: |
| ZcashAccountPubKey::derive_external_ivk() |       |       |       |       |       |
| ZcashAccountPubKey::derive_internal_ivk() |       |       |       |       |       |
| ZcashAccountPubKey::ovks_for_shielding()  |       |       |       |       |       |
| ZcashAccountPubKey::internal_ovk()        |       |       |       |       |       |
| ZcashAccountPubKey::external_ovk()        |       |       |       |       |       |
| ZcashAccountPubKey::serialize()           |       |       |       |       |       |
| ZcashAccountPubKey::deserialize()         |       |       |       |       |       |

### ZcashUnifiedSpendingKey

* Original type: [zcash_client_backend::keys::UnifiedSpendingKey]()

| Object/Method name                                                                                                    |    Score     |        UDL         |        Code        |       Tests        | Docs  |
| --------------------------------------------------------------------------------------------------------------------- | :----------: | :----------------: | :----------------: | :----------------: | :---: |
| ZcashUnifiedSpendingKey::from_seed()                                                                                  | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashUnifiedSpendingKey::to_unified_full_viewing_key() -> [ZcashUnifiedFullViewingKey](###ZcashUnifiedFullViewingKey) | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashUnifiedSpendingKey::transparent()                                                                                | :red_circle: |                    |                    |                    |       |
| ZcashUnifiedSpendingKey::sapling()                                                                                    | :red_circle: |                    |                    |                    |       |
| ZcashUnifiedSpendingKey::orchard()                                                                                    | :red_circle: |                    |                    |                    |       |
| ZcashUnifiedSpendingKey::to_bytes()                                                                                   | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashUnifiedSpendingKey::from_bytes()                                                                                 |              |                    |                    |                    |       |

### ZcashUnifiedFullViewingKey

* Original type: [zcash_client_backend::keys::UnifiedFullViewingKey]()

| Object/Method name                                                                                                         |    Score     |        UDL         |        Code        |       Tests        |        Docs        |
| -------------------------------------------------------------------------------------------------------------------------- | :----------: | :----------------: | :----------------: | :----------------: | :----------------: |
| ZcashUnifiedFullViewingKey::new()                                                                                          |              |                    |                    |                    |                    |
| ZcashUnifiedFullViewingKey::decode()                                                                                       | :red_circle: | :white_check_mark: | :white_check_mark: |                    |                    |
| ZcashUnifiedFullViewingKey::encode()                                                                                       | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |                    |
| ZcashUnifiedFullViewingKey::transparent()                                                                                  | :red_circle: |                    |                    |                    |                    |
| ZcashUnifiedFullViewingKey::sapling() -> [ZcashDiversifiableFullViewingKey](###ZcashDiversifiableFullViewingKey (Sapling)) | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |                    |
| ZcashUnifiedFullViewingKey::orchard()  -> [ZcashOrchardFullViewingKey](###ZcashOrchardFullViewingKey)                      | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| ZcashUnifiedFullViewingKey::address()                                                                                      |              |                    |                    |                    |                    |
| ZcashUnifiedFullViewingKey::find_address()                                                                                 |              |                    |                    |                    |                    |
| ZcashUnifiedFullViewingKey::default_address()                                                                              |              |                    |                    |                    |                    |

### ZcashDiversifiableFullViewingKey (Sapling)

* Original type: [zcash_client_backend::keys::sapling::DiversifiableFullViewingKey]()

| Object/Method name                                                                            |    Score     |        UDL         |        Code        |       Tests        | Docs  |
| --------------------------------------------------------------------------------------------- | :----------: | :----------------: | :----------------: | :----------------: | :---: |
| ZcashDiversifiableFullViewingKey::from_bytes()                                                | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |
| ZcashDiversifiableFullViewingKey::to_bytes()                                                  | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |
| ZcashDiversifiableFullViewingKey::fvk()                                                       | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |
| ZcashDiversifiableFullViewingKey::to_nk()                                                     | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |
| ZcashDiversifiableFullViewingKey::to_ivk() -> [ZcashSaplingIvk](###ZcashSaplingIvk (Sapling)) | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashDiversifiableFullViewingKey::to_ovk()                                                    | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |
| ZcashDiversifiableFullViewingKey::address()                                                   | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |
| ZcashDiversifiableFullViewingKey::find_address()                                              | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |
| ZcashDiversifiableFullViewingKey::default_address()                                           | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |
| ZcashDiversifiableFullViewingKey::diversified_address()                                       | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |
| ZcashDiversifiableFullViewingKey::change_address()                                            | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |
| ZcashDiversifiableFullViewingKey::diversified_change_address()                                | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |
| ZcashDiversifiableFullViewingKey::decrypt_diversifier()                                       | :red_circle: | :white_check_mark: | :white_check_mark: |                    |       |

### ZcashExtendedSpendingKey (Sapling)

* Original type: [zcash_primitives::zip32::sapling::ExtendedSpendingKey](https://docs.rs/zcash_primitives/0.10.0/zcash_primitives/zip32/sapling/struct.ExtendedSpendingKey.html)

| Object/Method name                                            |    Score     |        UDL         |        Code        |       Tests        | Docs  |
| ------------------------------------------------------------- | :----------: | :----------------: | :----------------: | :----------------: | :---: |
| ZcashExtendedSpendingKey::master()                            | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashExtendedSpendingKey::from_bytes()                        | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashExtendedSpendingKey::read()                              |              |                    |                    |                    |       |
| ZcashExtendedSpendingKey::to_bytes()                          | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashExtendedSpendingKey::write()                             |              |                    |                    |                    |       |
| ZcashExtendedSpendingKey::from_path()                         | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashExtendedSpendingKey::derive_child()                      | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashExtendedSpendingKey::default_address()                   | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashExtendedSpendingKey::derive_internal()                   | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashExtendedSpendingKey::to_extended_full_viewing_key()      |              |                    |                    |                    |       |
| ZcashExtendedSpendingKey::to_diversifiable_full_viewing_key() | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |

### ZcashSaplingIvk (Sapling)

* Original type: [zcash_primitives::sapling::SaplingIvk](https://docs.rs/zcash_primitives/0.10.0/zcash_primitives/sapling/index.html#reexport.SaplingIvk)

| Object/Method name                                                                                                       |    Score     |        UDL         |        Code        |       Tests        | Docs  |
| ------------------------------------------------------------------------------------------------------------------------ | :----------: | :----------------: | :----------------: | :----------------: | :---: |
| ::to_payment_address([ZcashDiversifier](###ZcashDiversifier)) -> [ZcashPaymentAddress](###ZcashPaymentAddress (Sapling)) | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashSaplingIvk::to_repr()                                                                                               |              | :white_check_mark: | :white_check_mark: |                    |       |

### ZcashDiversifier (Sapling)

* Original type: [zcash_primitives::sapling::Diversifier](https://docs.rs/zcash_primitives/latest/zcash_primitives/sapling/keys/struct.Diversifier.html)

| Object/Method name      |    Score     |        UDL         |        Code        |       Tests        | Docs  |
| ----------------------- | :----------: | :----------------: | :----------------: | :----------------: | :---: |
| ::new()                 | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashDiversifier::g_d() |              |                    |                    |                    |       |

### ZcashOutgoingViewingKey (Sapling)

* Working on this :hammer:

Original type: [zcash_primitives::sapling::keys::OutgoingViewingKey](https://docs.rs/zcash_primitives/latest/zcash_primitives/sapling/keys/struct.OutgoingViewingKey.html)

This is partially defined in `UDL` and `code`.

| Object/Method name                  |    Score     |        UDL         |        Code        |       Tests        |        Docs        |
| ----------------------------------- | :----------: | :----------------: | :----------------: | :----------------: | :----------------: |
| ZcashOutgoingViewingKey::to_bytes() | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |

### ZcashPaymentAddress (Sapling)

* Original type: [zcash_primitives::sapling::PaymentAddress](https://docs.rs/zcash_primitives/latest/zcash_primitives/sapling/struct.PaymentAddress.html)

| Object/Method name                 |    Score     |        UDL         |        Code        |       Tests        | Docs  |
| ---------------------------------- | :----------: | :----------------: | :----------------: | :----------------: | :---: |
| ZcashPaymentAddress::from_parts()  |              |                    |                    |                    |       |
| ZcashPaymentAddress::from_bytes()  |              |                    |                    |                    |       |
| ZcashPaymentAddress::to_bytes()    | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |       |
| ZcashPaymentAddress::diversifier() |              |                    |                    |                    |       |
| ZcashPaymentAddress::pk_d()        |              |                    |                    |                    |       |
| ZcashPaymentAddress::g_d()         |              |                    |                    |                    |       |
| ZcashPaymentAddress::create_note() |              |                    |                    |                    |       |


### ZcashOrchardSpendingKey

* Original type: [orchard::keys::SpendingKey](https://docs.rs/orchard/0.3.0/orchard/keys/struct.SpendingKey.html)

| Object/Method name                         | Score        | UDL                | Code               | Tests              | Docs |
| ------------------------------------------ | ------------ | ------------------ | ------------------ | ------------------ | ---- |
| ZcashOrchardSpendingKey::to_bytes()        | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |      |
| ZcashOrchardSpendingKey::from_zip32_seed() | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |      |
| ZcashOrchardSpendingKey::from_bytes()      | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: |      |


### ZcashOrchardFullViewingKey

* Original type: [orchard::keys::FullViewingKey](https://docs.rs/orchard/0.3.0/orchard/keys/struct.FullViewingKey.html)

| Object/Method name                                                                                          | Score        | UDL                | Code               | Tests              | Docs               |
| ----------------------------------------------------------------------------------------------------------- | ------------ | ------------------ | ------------------ | ------------------ | ------------------ |
| ZcashOrchardFullViewingKey::derive_dk_ovk()                                                                 |              |                    |                    |                    |                    |
| ZcashOrchardFullViewingKey::address_at()                                                                    |              |                    |                    |                    |                    |
| ZcashOrchardFullViewingKey::address()                                                                       |              |                    |                    |                    |                    |
| ZcashOrchardFullViewingKey::scope_for_address()                                                             |              |                    |                    |                    |                    |
| ZcashOrchardFullViewingKey::write()                                                                         |              |                    |                    |                    |                    |
| ZcashOrchardFullViewingKey::read()                                                                          |              |                    |                    |                    |                    |
| ZcashOrchardFullViewingKey::to_bytes()                                                                      |              |                    |                    |                    |                    |
| ZcashOrchardFullViewingKey::from_bytes()                                                                    |              |                    |                    |                    |                    |
| ZcashOrchardFullViewingKey::to_ivk() -> [ZcashOrchardIncomingViewingKey](###ZcashOrchardIncomingViewingKey) | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| ZcashOrchardFullViewingKey::to_ovk() -> [ZcashOrchardOutgoingViewingKey](###ZcashOrchardOutgoingViewingKey) | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |

### ZcashOrchardIncomingViewingKey

* Original type: [orchard::keys::IncomingViewingKey](https://docs.rs/orchard/0.3.0/orchard/keys/struct.IncomingViewingKey.html)

| Object/Method name                                                                          | Score        | UDL                | Code               | Tests              | Docs               |
| ------------------------------------------------------------------------------------------- | ------------ | ------------------ | ------------------ | ------------------ | ------------------ |
| ZcashOrchardIncomingViewingKey::to_bytes()                                                  | :red_circle: |                    |                    |                    |                    |
| ZcashOrchardIncomingViewingKey::from_bytes()                                                |              |                    |                    |                    |                    |
| ZcashOrchardIncomingViewingKey::diversifier_index()                                         |              |                    |                    |                    |                    |
| ZcashOrchardIncomingViewingKey::address_at()                                                |              |                    |                    |                    |                    |
| ZcashOrchardIncomingViewingKey::address()  -> [ZcashOrchardAddress](###ZcashOrchardAddress) | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |

### ZcashOrchardOutgoingViewingKey

* Original type: [orchard::keys::OutgoingViewingKey](https://docs.rs/orchard/0.3.0/orchard/keys/struct.OutgoingViewingKey.html)
* Just a type, no pub methods

| Object/Method name | Score | UDL | Code | Tests | Docs |
| ------------------ | ----- | --- | ---- | ----- | ---- |

### ZcashOrchardAddress

* Original type: [orchard::Address](https://docs.rs/orchard/0.3.0/orchard/struct.Address.html)

| Object/Method name                            | Score        | UDL                | Code               | Tests              | Docs               |
| --------------------------------------------- | ------------ | ------------------ | ------------------ | ------------------ | ------------------ |
| ZcashOrchardAddress::diversifier()            |              |                    |                    |                    |                    |
| ZcashOrchardAddress::to_raw_address_bytes()   | :red_circle: | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| ZcashOrchardAddress::from_raw_address_bytes() |              |                    |                    |                    |                    |