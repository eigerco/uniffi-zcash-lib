import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()


fun testExtendedPrivKeyFromBytes() {
    val bytes = supp.getAsU8Array("hdwallet_epk")

	val zepk = ZcashExtendedPrivKey.fromBytes(bytes)

    assert(zepk.toBytes() == bytes)
}

testExtendedPrivKeyFromBytes()

fun testExtendedPrivKeyFromRandom() {
	val zepk = ZcashExtendedPrivKey.random()

	// no error thrown
}
testExtendedPrivKeyFromRandom()

fun testExtendedPrivKeyFromRandomWithSeedSize() {
	val seedSize = ZcashKeySeed.S128
	val zepk = ZcashExtendedPrivKey.randomWithSeedSize(seedSize)

	// no errors occurred
}
testExtendedPrivKeyFromRandomWithSeedSize()

fun testExtendedPrivKeyFromWithSeed() {
	val seed = supp.getAsU8Array("seed")
	val zepk = ZcashExtendedPrivKey.withSeed(seed)

    val bytes = supp.getAsU8Array("hdwallet_epk")

    assert(zepk.toBytes() == bytes)
}
testExtendedPrivKeyFromWithSeed()

fun testExtendedPrivKeyDerivePrivateKey() {
	val seed = supp.getAsU8Array("seed")
	val zepk = ZcashExtendedPrivKey.withSeed(seed)
	val idx = ZcashKeyIndex.fromIndex(3u)

    val expected = supp.getAsU8Array("hdwallet_epk_derive_private_key")

	assert(zepk.derivePrivateKey(idx).toBytes() == expected)
}
testExtendedPrivKeyDerivePrivateKey()


fun testKeyIndexFromU32() {
    val zidx = ZcashKeyIndex.fromIndex(3u)


	// no errors happened
}
testKeyIndexFromU32()

fun testKeyIndexFromIndex() {
	val zidx = ZcashKeyIndex.fromIndex(3u)

	// no errors happened
}
testKeyIndexFromIndex()

fun testKeyIndexRawIndex() {
	val zidx = ZcashKeyIndex.fromIndex(3u)

	assert(zidx.rawIndex() == 3u)
}
testKeyIndexRawIndex()

fun testKeyIndexNormalizeIndex() {
	val zidx = ZcashKeyIndex.fromIndex(2147483648u + 3u)

	assert(zidx.normalizeIndex() == 3u)
}
testKeyIndexNormalizeIndex()

fun testKeyIndexHardenedFromNormalizeIndex() {
	val zidx = ZcashKeyIndex.hardenedFromNormalizeIndex(3u)

    assert(zidx.rawIndex() == 2147483648u + 3u)
	assert(zidx.normalizeIndex() == 3u)

}
testKeyIndexHardenedFromNormalizeIndex()

fun testKeyIndexIsValid() {
	val zidx = ZcashKeyIndex.fromIndex(3u)

	assert(zidx.isValid() == true)
}
testKeyIndexIsValid()



fun testOrchardAddressFromRawAddressBytes() {
	val rawBytes = supp.getAsU8Array("orchard_address")
	val zoa = ZcashOrchardAddress.fromRawAddressBytes(rawBytes)

	assert(zoa.toRawAddressBytes() == rawBytes)
}
testOrchardAddressFromRawAddressBytes()

fun testOrchardAddressDiversifier() {
	val rawBytes = supp.getAsU8Array("orchard_address")
	val expectedBytes = supp.getAsU8Array("orchard_diversifier")
	val zoa = ZcashOrchardAddress.fromRawAddressBytes(rawBytes)
	val diver = zoa.diversifier().toBytes()

	assert(diver == expectedBytes)
}
testOrchardAddressDiversifier()

fun testOrchardIvkToPaymentAddress() {
	val seed = supp.getAsU8Array("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val orchardDiversifier = ZcashOrchardDiversifier.fromBytes(listOf(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() })

    val expectedBytes = supp.getAsU8Array("orchard_address")

    val rawAddressBytes = unifiedSpendingKey.toUnifiedFullViewingKey()
        .orchard()!!.toIvk(ZcashOrchardScope.EXTERNAL)
        .address(orchardDiversifier)!!.toRawAddressBytes()

    assert(rawAddressBytes == expectedBytes)
}
testOrchardIvkToPaymentAddress()



fun testOrchardDiversifierIndexFromBytes() {
	val expectedBytes = supp.getAsU8Array("orchard_diversifier_index_from_u32")
	val divIdx = ZcashOrchardDiversifierIndex.fromBytes(expectedBytes)

	assert(divIdx.toBytes() == expectedBytes)
}
testOrchardDiversifierIndexFromBytes()

fun testOrchardDiversifierIndexFromU32() {
	val integer = supp.getAsU32("orchard_diversifier_index_u32")
	val expectedBytes = supp.getAsU8Array("orchard_diversifier_index_from_u32")

	val divIdx = ZcashOrchardDiversifierIndex.fromU32(integer)

	assert(divIdx.toBytes() == expectedBytes)
}
testOrchardDiversifierIndexFromU32()

fun testOrchardDiversifierIndexFromU64() {
	val integer = supp.getAsU64("orchard_diversifier_index_u64");
	val expectedBytes = supp.getAsU8Array("orchard_diversifier_index_from_u64")

	val divIdx = ZcashOrchardDiversifierIndex.fromU64(integer)

	assert(divIdx.toBytes() == expectedBytes)
}
testOrchardDiversifierIndexFromU64()




fun testOrchardDiversifierFromBytes() {
	val expectedBytes = supp.getAsU8Array("orchard_diversifier")

	val zod = ZcashOrchardDiversifier.fromBytes(expectedBytes)

	assert(expectedBytes == zod.toBytes())
}
testOrchardDiversifierFromBytes()



fun setupGetFvk() =
    ZcashOrchardFullViewingKey.fromBytes(supp.getAsU8Array("orchard_full_viewing_key"))

fun testOrchardFullViewingKeyToBytes() {
    val bytes = supp.getAsU8Array("orchard_full_viewing_key");

    val key = ZcashOrchardFullViewingKey.fromBytes(bytes)

    assert(key.toBytes() == bytes)
}
testOrchardFullViewingKeyToBytes()

fun testOrchardFullViewingKeyAddressAt() {
	val fvk = setupGetFvk()
	val divIdx = ZcashOrchardDiversifierIndex.fromU32(4u)
	val expectedBytes = supp.getAsU8Array("orchard_div_idx_address_at")
	val addr = fvk.addressAt(divIdx, ZcashOrchardScope.EXTERNAL)

	assert(addr.toRawAddressBytes() == expectedBytes)
}
testOrchardFullViewingKeyAddressAt()

fun testOrchardFullViewingKeyAddress() {
	val ofvkBytes = supp.getAsU8Array("orchard_full_viewing_key")
	val fvk = ZcashOrchardFullViewingKey.fromBytes(ofvkBytes)
	val divBytes = supp.getAsU8Array("orchard_diversifier")
	val expectedBytes = supp.getAsU8Array("orchard_div_idx_address")
	val diver = ZcashOrchardDiversifier.fromBytes(divBytes)
	val addr = fvk.address(diver, ZcashOrchardScope.EXTERNAL)

	assert(addr.toRawAddressBytes() == expectedBytes)
}
testOrchardFullViewingKeyAddress()

fun testOrchardFullViewingKeyScopeForAddress() {
	val fvk = setupGetFvk()
	val divBytes = supp.getAsU8Array("orchard_diversifier")
	val diver = ZcashOrchardDiversifier.fromBytes(divBytes)
	val addr = fvk.address(diver, ZcashOrchardScope.EXTERNAL)

	assert(fvk.scopeForAddress(addr) == ZcashOrchardScope.EXTERNAL)
}
testOrchardFullViewingKeyScopeForAddress()

fun testOrchardFullViewingKeyToIvk() {

	val fvk = setupGetFvk()
	val ivk = fvk.toIvk(ZcashOrchardScope.EXTERNAL)
	val expectedBytes = supp.getAsU8Array("orchard_full_viewing_key_ivk")

	assert(ivk.toBytes() == expectedBytes)
}
testOrchardFullViewingKeyToIvk()

fun testOrchardFullViewingKeyToOvk() {
	val fvk = setupGetFvk()
	val ovk = fvk.toOvk(ZcashOrchardScope.EXTERNAL)
	val expectedBytes = supp.getAsU8Array("orchard_full_viewing_key_ovk")

	assert(ovk.toBytes() == expectedBytes)
}
testOrchardFullViewingKeyToOvk()



fun testOrchardIncomingViewingKeyToBytes() {
    val bytes = supp.getAsU8Array("orchard_full_viewing_key_ivk");

    val key = ZcashOrchardIncomingViewingKey.fromBytes(bytes)

    assert(key.toBytes() == bytes)
}
testOrchardIncomingViewingKeyToBytes()

fun testOrchardIncomingViewingKeyDiversifierIndex() {
    val bytes = supp.getAsU8Array("orchard_full_viewing_key_ivk");

    val key = ZcashOrchardIncomingViewingKey.fromBytes(bytes)

    val index = ZcashOrchardDiversifierIndex.fromU32(0u)

    val address = key.addressAt(index)

    assert(key.diversifierIndex(address)!!.toBytes() == index.toBytes())
}
testOrchardIncomingViewingKeyDiversifierIndex()

fun testOrchardIncomingViewingKeyAddressAt() {
    val bytes = supp.getAsU8Array("orchard_full_viewing_key_ivk");

    val key = ZcashOrchardIncomingViewingKey.fromBytes(bytes)

    val index = ZcashOrchardDiversifierIndex.fromU32(0u)

    val address = key.addressAt(index)

    val expected = supp.getAsU8Array("orchard_incoming_viewing_key_address_at")

    assert(address.toRawAddressBytes() == expected)
}
testOrchardIncomingViewingKeyAddressAt()

fun testOrchardIncomingViewingKeyAddress() {
    val bytes = supp.getAsU8Array("orchard_full_viewing_key_ivk");


    val key = ZcashOrchardIncomingViewingKey.fromBytes(bytes)

	val zod = ZcashOrchardDiversifier.fromBytes(supp.getAsU8Array("orchard_diversifier"))

    val address = key.address(zod)

    val expected = supp.getAsU8Array("orchard_incoming_viewing_key_address")

    assert(address.toRawAddressBytes() == expected)
}
testOrchardIncomingViewingKeyAddress()



fun testOrchardOutgoingViewingKeyFromBytes() {
    val bytes = supp.getAsU8Array("orchard_outgoing_viewing_key");

    val key = ZcashOrchardOutgoingViewingKey.fromBytes(bytes)

    assert(key.toBytes() == bytes)
}
testOrchardOutgoingViewingKeyFromBytes()



fun testOrchardSpendingKeyFromBytes() {
    val keyBytes = supp.getAsU8Array("orchard_spending_key")

    val key = ZcashOrchardSpendingKey.fromBytes(keyBytes)

    assert(key.toBytes() == keyBytes)
}
testOrchardSpendingKeyFromBytes()

fun testOrchardSpendingKeyToFvk() {
    val keyBytes = supp.getAsU8Array("orchard_spending_key")

    val key = ZcashOrchardSpendingKey.fromBytes(keyBytes)

    val expectedBytes = supp.getAsU8Array("orchard_spending_key_fvk")

    assert(key.toFvk().toBytes() == expectedBytes)
}
testOrchardSpendingKeyToFvk()

fun testOrchardSpendingKeyArrayMismatch() {
    val keyBytes = listOf(0, 1).map { it.toUByte() }

    var thrown = false;
    try {
        ZcashOrchardSpendingKey.fromBytes(keyBytes)
    } catch (e: ZcashException.ArrayLengthMismatch) {
        thrown = true;
    }
    assert(thrown)
}
testOrchardSpendingKeyArrayMismatch()

fun testOrchardSpendingKeyFromZip32Seed() {
    val seed = supp.getAsU8Array("seed")
    val coinType = supp.getAsU32("coin_type")
    val account = supp.getAsU32("account")

    val keyExpectedBytes = supp.getAsU8Array("orchard_spending_key_from_zip32_seed")

    val key = ZcashOrchardSpendingKey.fromZip32Seed(seed, coinType, account)

    assert(key.toBytes() == keyExpectedBytes)
}
testOrchardSpendingKeyFromZip32Seed()



fun testSecpSecretKey() {
	val expectedBytes = supp.getAsU8Array("secp_secret_key")
	val ssk = SecpSecretKey(expectedBytes)

	assert(ssk.serializeSecret() == expectedBytes)
}
testSecpSecretKey()



fun testAllGetters() {
	assert(supp.getAsU8Array("seed") == listOf(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() })
	assert(supp.getAsU32Array("seed") == listOf(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUInt() })
    assert(supp.getAsU64Array("seed") == listOf(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toULong() })
	assert(supp.getAsU32("coin_type") == 234u)
    assert(supp.getAsU64("coin_type") == 234.toULong())
	assert(supp.getAsString("test_string") == "TestString")
}
testAllGetters()



fun testRecipientAddressShielded() {
    val params = ZcashConsensusParameters.MAIN_NETWORK

    val source = supp.getAsString("recipient_address_shielded_source")

    val shielded = ZcashPaymentAddress.decode(params, source)

    val address = ZcashRecipientAddress.shielded(shielded)

    val expected = supp.getAsString("recipient_address_shielded")

    assert(address.encode(params) == expected)
}
testRecipientAddressShielded()

fun testRecipientAddressTransparent() {
    val params = ZcashConsensusParameters.MAIN_NETWORK

    val source = supp.getAsString("recipient_address_transparent_source")

    val transparent = ZcashTransparentAddress.decode(params, source)

    val address = ZcashRecipientAddress.transparent(transparent)

    val expected = supp.getAsString("recipient_address_transparent")

    assert(address.encode(params) == expected)
}
testRecipientAddressTransparent()

fun testRecipientAddressUnified() {
    val params = ZcashConsensusParameters.MAIN_NETWORK

    val source = supp.getAsString("recipient_address_unified_source")

    val unified = ZcashUnifiedAddress.decode(params, source)

    val address = ZcashRecipientAddress.unified(unified)

    val expected = supp.getAsString("recipient_address_unified")

    assert(address.encode(params) == expected)
}
testRecipientAddressUnified()

fun testRecipientAddressDecode() {
    val expected = supp.getAsString("recipient_address_unified")

    val params = ZcashConsensusParameters.MAIN_NETWORK

    val address = ZcashRecipientAddress.decode(params, expected)

    assert(address.encode(params) == expected)
}
testRecipientAddressDecode()



fun testUnifiedAddressParsing() {
    val seed = supp.getAsU8Array("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val params = ZcashConsensusParameters.MAIN_NETWORK

    var thrown = false;
    try {
        ZcashUnifiedAddress.decode(params, "")
    } catch (e: ZcashException.Message) {
        thrown = true;
    }
    assert(thrown)

    val diversifierBytes = supp.getAsU8Array("diversifier")

    val saplingDiversifier = ZcashDiversifier(diversifierBytes)

    val sapling = unifiedSpendingKey.toUnifiedFullViewingKey()
        .sapling()!!.toIvk(ZcashScope.EXTERNAL)
        .toPaymentAddress(saplingDiversifier)

    val orchardDiversifier = ZcashOrchardDiversifier.fromBytes(diversifierBytes)

    val orchard = unifiedSpendingKey.toUnifiedFullViewingKey()
        .orchard()!!.toIvk(ZcashOrchardScope.EXTERNAL)
        .address(orchardDiversifier)

    val transparentAddressPublicKey = supp.getAsU8Array("transparent_address_public_key")

    val transparent = ZcashTransparentAddress.fromPublicKey(transparentAddressPublicKey)

    val source = ZcashUnifiedAddress(orchard, sapling, transparent)
    val address = source.encode(params)
    val parsed = ZcashUnifiedAddress.decode(params, address)

    assert(address == parsed.encode(params))
}
testUnifiedAddressParsing()

fun testUnifiedAddressCreationWithSapling() {
    val seed = supp.getAsU8Array("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,

            seed,
        ZcashAccountId(0u),
    )

    val diversifierBytes = supp.getAsU8Array("diversifier")

    val saplingDiversifier = ZcashDiversifier(diversifierBytes)

    val sapling = unifiedSpendingKey.toUnifiedFullViewingKey()
        .sapling()!!.toIvk(ZcashScope.EXTERNAL)
        .toPaymentAddress(saplingDiversifier)

    var unifiedAddress = ZcashUnifiedAddress(null, sapling, null)

    assert(sapling!!.toBytes() == unifiedAddress.sapling()!!.toBytes())
    assert(null == unifiedAddress.orchard())
}
testUnifiedAddressCreationWithSapling()

fun testUnifiedAddressCreation() {
    val seed = supp.getAsU8Array("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val diversifierBytes = supp.getAsU8Array("diversifier")

    val saplingDiversifier = ZcashDiversifier(diversifierBytes)

    val sapling = unifiedSpendingKey.toUnifiedFullViewingKey()
        .sapling()!!.toIvk(ZcashScope.EXTERNAL)
        .toPaymentAddress(saplingDiversifier)

    val orchardDiversifier = ZcashOrchardDiversifier.fromBytes(diversifierBytes)

    val orchard = unifiedSpendingKey.toUnifiedFullViewingKey()
        .orchard()!!.toIvk(ZcashOrchardScope.EXTERNAL)
        .address(orchardDiversifier)

    val transparentAddressPublicKey = supp.getAsU8Array("transparent_address_public_key")

    val transparent = ZcashTransparentAddress.fromPublicKey(transparentAddressPublicKey)

    // At least one of orchard or sapling address must be set
    // ZcashUnifiedAddress(null, null, null)
    ZcashUnifiedAddress(orchard, null, null)
    ZcashUnifiedAddress(null, sapling, null)
    ZcashUnifiedAddress(orchard, sapling, null)
    // ZcashUnifiedAddress(null, null, transparent)
    ZcashUnifiedAddress(orchard, null, transparent)
    ZcashUnifiedAddress(null, sapling, transparent)
    ZcashUnifiedAddress(orchard, sapling, transparent)
}
testUnifiedAddressCreation()

fun testUnifiedAddressCreationWithOrchard() {
    val seed = supp.getAsU8Array("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val diversifierBytes = supp.getAsU8Array("diversifier")

    val orchardDiversifier = ZcashOrchardDiversifier.fromBytes(diversifierBytes)

    val orchard = unifiedSpendingKey.toUnifiedFullViewingKey()
        .orchard()!!.toIvk(ZcashOrchardScope.EXTERNAL)
        .address(orchardDiversifier)

    val unifiedAddress = ZcashUnifiedAddress(orchard, null, null)

    assert(null == unifiedAddress.sapling())
    assert(orchard.toRawAddressBytes() == unifiedAddress.orchard()!!.toRawAddressBytes())
}
testUnifiedAddressCreationWithOrchard()



fun testDecodeExtendedFullViewingKey() {
	val hrp = supp.getAsString("hrp_efvk")

	val fvk = supp.getAsString("extended_fvk_encoded")


	val decoded = decodeExtendedFullViewingKey(hrp, fvk)

	assert(encodeExtendedFullViewingKey(hrp, decoded) == fvk)
}
testDecodeExtendedFullViewingKey()

fun testDecodeExtendedSpendingKey() {
	val hrp = supp.getAsString("hrp_esk")

    val encoded = supp.getAsString("esk_encoded")

    val key = decodeExtendedSpendingKey(hrp, encoded)

    assert(encodeExtendedSpendingKey(hrp, key) == encoded)
}
testDecodeExtendedSpendingKey()

fun testDecodePaymentAddress() {
    val expected = supp.getAsU8Array("viewing_key_payment_address")

    val address = ZcashPaymentAddress.fromBytes(expected)

    val hrp = supp.getAsString("hrp_payment_address")

    val encoded = encodePaymentAddress(hrp, address)

    val decoded = decodePaymentAddress(hrp, encoded)

    assert(decoded.toBytes() == expected)
}
testDecodePaymentAddress()

fun testDecodeTransparentAddress() {
    val expected = supp.getAsString("t_address_script")

    val pubkey = supp.getAsU8Array("b58_pubkey_address_prefix")

    val script = supp.getAsU8Array("b58_script_address_prefix")

    val decoded = decodeTransparentAddress(pubkey, script, expected)

    val encoded = encodeTransparentAddress(pubkey, script, decoded)

    assert(encoded == expected)
}
testDecodeTransparentAddress()

fun testEncodePaymentAddressP() {
    val expected = supp.getAsU8Array("viewing_key_payment_address")

    val address = ZcashPaymentAddress.fromBytes(expected)

    val params = ZcashConsensusParameters.MAIN_NETWORK

    val encoded = encodePaymentAddressP(params, address)

    val hrp = supp.getAsString("hrp_payment_address")

    val decoded = decodePaymentAddress(hrp, encoded)

    assert(decoded.toBytes() == expected)
}
testEncodePaymentAddressP()

fun testEncodeTransparentAddressP() {
    val expected = supp.getAsString("t_address_script")

    val pubkey = supp.getAsU8Array("b58_pubkey_address_prefix")

    val script = supp.getAsU8Array("b58_script_address_prefix")

    val decoded = decodeTransparentAddress(pubkey, script, expected)

    val params = ZcashConsensusParameters.TEST_NETWORK

    val encoded = encodeTransparentAddressP(params, decoded)

    assert(encoded == expected)
}
testEncodeTransparentAddressP()



fun testUnifiedFullViewingKeyNew() {
    val encoded = supp.getAsString("unified_full_viewing_key_encoded")

    val params = ZcashConsensusParameters.MAIN_NETWORK

    val key = ZcashUnifiedFullViewingKey.decode(params, encoded)

    val transparent = key.transparent();
    val sapling = key.sapling();
    val orchard = key.orchard();

    val key_from_parts = ZcashUnifiedFullViewingKey(transparent, sapling, orchard)

    assert(key_from_parts.encode(params) == encoded)
}
testUnifiedFullViewingKeyNew()

fun testUnifiedFullViewingKeyDecode() {
    val encoded = supp.getAsString("unified_full_viewing_key_encoded")

    val params = ZcashConsensusParameters.MAIN_NETWORK

    val key = ZcashUnifiedFullViewingKey.decode(params, encoded)

    assert(key.encode(params) == encoded)
}
testUnifiedFullViewingKeyDecode()

fun testUnifiedFullViewingKeyEncode() {
    val seed = supp.getAsU8Array("seed")
    val params = ZcashConsensusParameters.MAIN_NETWORK

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        params,
        seed,
        ZcashAccountId(0u),
    )

    val expected = supp.getAsString("unified_full_viewing_key_encoded")

    val encodedSk = unifiedSpendingKey.toUnifiedFullViewingKey().encode(params)

    assert(encodedSk == expected)
}
testUnifiedFullViewingKeyEncode()

fun testUnifiedFullViewingKeyAddress() {
    val encoded = supp.getAsString("unified_full_viewing_key_encoded_2")

    val params = ZcashConsensusParameters.MAIN_NETWORK

    val key = ZcashUnifiedFullViewingKey.decode(params, encoded)

    val expected = supp.getAsString("unified_full_viewing_key_address_encoded");

    val index = ZcashDiversifierIndex.fromU32(0u)

    assert(key.address(index)!!.encode(params) == expected)
}
testUnifiedFullViewingKeyAddress()

fun testUnifiedFullViewingKeyFindAddress() {
    val encoded = supp.getAsString("unified_full_viewing_key_encoded_2")

    val params = ZcashConsensusParameters.MAIN_NETWORK

    val key = ZcashUnifiedFullViewingKey.decode(params, encoded)

    val index = ZcashDiversifierIndex.fromU32(0u)

    val foundAddress = key.findAddress(index)!!

    val expectedAddress = supp.getAsString("unified_full_viewing_key_find_address_address_encoded");

    val expectedIndex = supp.getAsU8Array("unified_full_viewing_key_find_address_index");

    assert(foundAddress.address.encode(params) == expectedAddress)
    assert(foundAddress.diversifierIndex.toBytes() == expectedIndex)
}
testUnifiedFullViewingKeyFindAddress()

fun testUnifiedFullViewingKeyDefaultAddress() {
    val encoded = supp.getAsString("unified_full_viewing_key_encoded_2")

    val params = ZcashConsensusParameters.MAIN_NETWORK

    val key = ZcashUnifiedFullViewingKey.decode(params, encoded)

    val defaultAddress = key.defaultAddress()

    val expectedAddress = supp.getAsString("unified_full_viewing_key_default_address_address_encoded");

    val expectedIndex = supp.getAsU8Array("unified_full_viewing_key_default_address_index");

    assert(defaultAddress.address.encode(params) == expectedAddress)
    assert(defaultAddress.diversifierIndex.toBytes() == expectedIndex)
}
testUnifiedFullViewingKeyDefaultAddress()



fun testUnifiedSpendingKeyFromSeed() {
    val seed = supp.getAsU8Array("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = supp.getAsU8Array("unified_spending_key")

    assert(unifiedSpendingKey.toBytes(ZcashKeysEra.ORCHARD) == expected)
}
testUnifiedSpendingKeyFromSeed()


fun testSpendingKeyConversions() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val expectedKeyBytes = supp.getAsU8Array("extended_spending_key")

    assert(key.toBytes() == expectedKeyBytes)
}
testSpendingKeyConversions()

fun testSpendingKeyFromPath(){
    val seed = supp.getAsU8Array("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val childIndex = listOf(
            ZcashChildIndex.Hardened(32u),
            ZcashChildIndex.Hardened(133u),
            ZcashChildIndex.Hardened(2u),
            ZcashChildIndex.NonHardened(3u),
    )

    val derivedKey = ZcashExtendedSpendingKey.fromPath(key, childIndex)

    val expectedDerivedKeyBytes = supp.getAsU8Array("extended_spending_key_from_path")
    assert(derivedKey.toBytes() == expectedDerivedKeyBytes)
}
testSpendingKeyFromPath()

fun testSpendingKeyDeriveChild() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val derivedKey = key.deriveChild(ZcashChildIndex.Hardened(32u))

    val expectedDerivedKeyBytes = supp.getAsU8Array("extended_spending_key_derived_child")
    assert(derivedKey.toBytes() == expectedDerivedKeyBytes)
}
testSpendingKeyDeriveChild()

fun testSpendingKeyDefaultAddress() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val result = key.defaultAddress()

    val expectedAddressBytes = supp.getAsU8Array("extended_spending_key_default_address")

    assert( result.address.toBytes() == expectedAddressBytes)

    val expectedIndexBytes = supp.getAsU8Array("extended_spending_key_child_index")

    assert(result.diversifierIndex.toBytes() == expectedIndexBytes)
}
testSpendingKeyDefaultAddress()

fun testSpendingKeyDeriveInternal(){
    val seed = supp.getAsU8Array("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val derivedKey = key.deriveInternal()

    val expectedDerivedKeyBytes = supp.getAsU8Array("extended_spending_key_internal_sk")

    assert(derivedKey.toBytes() == expectedDerivedKeyBytes)
}
testSpendingKeyDeriveInternal()

fun testSpendingKeyToDiversifiableFvk () {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val fvk = key.toDiversifiableFullViewingKey()

    val expectedFvkBytes = supp.getAsU8Array("diversifiable_fvk")

    assert(fvk.toBytes() == expectedFvkBytes)
}
testSpendingKeyToDiversifiableFvk()



fun testAccountPrivKeyFromSeed() {
    val seed = supp.getAsU8Array("seed")

    val accountPrivKey = ZcashAccountPrivKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = supp.getAsU8Array("account_private_key")
    assert(accountPrivKey.toBytes() == expected)
}
testAccountPrivKeyFromSeed()

fun testAccountPrivKeyFromExtendedPrivKey() {
    val seed = supp.getAsU8Array("seed")

    val extendedPrivKey = ZcashExtendedPrivKey.withSeed(seed)
    val accountPrivKey = ZcashAccountPrivKey.fromExtendedPrivkey(extendedPrivKey)

    val expected = supp.getAsU8Array("extended_private_key")
    assert(accountPrivKey.toBytes() == expected)
}
testAccountPrivKeyFromExtendedPrivKey()

fun testAccountPrivKeyToAccountPubKey() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashAccountPrivKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = supp.getAsU8Array("account_public_key")

    assert(key.toAccountPubkey().serialize() == expected)
}
testAccountPrivKeyToAccountPubKey()

fun testAccountPrivKeyDeriveExternalSecretKey() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashAccountPrivKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = supp.getAsU8Array("apk_derive_external_secret_key")

    assert(key.deriveExternalSecretKey(0u).serializeSecret() == expected)
}
testAccountPrivKeyDeriveExternalSecretKey()

fun testAccountPrivKeyDeriveInternalSecretKey() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashAccountPrivKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = supp.getAsU8Array("apk_derive_internal_secret_key")

    assert(key.deriveInternalSecretKey(0u).serializeSecret() == expected)
}
testAccountPrivKeyDeriveInternalSecretKey()

fun testAccountPrivKeyFromBytes() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashAccountPrivKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = key.toBytes()

    assert(ZcashAccountPrivKey.fromBytes(expected).toBytes() == expected)
}
testAccountPrivKeyFromBytes()



fun testAccountPubKeyNew() {
	val ppkBytes = supp.getAsU8Array("account_public_key")

	val ppk = ZcashAccountPubKey(ppkBytes)

	assert(ppk.serialize() == ppkBytes)
}
testAccountPubKeyNew()

fun testAccountPubKeyExternalIvk() {
	val ppkBytes = supp.getAsU8Array("account_public_key")
	val ivkBytes = supp.getAsU8Array("ppk_external_ivk")

	val ppk = ZcashAccountPubKey(ppkBytes)
	val ivk = ppk.deriveExternalIvk()

	assert(ivk.toBytes() == ivkBytes)
}
testAccountPubKeyExternalIvk()

fun testAccountPubKeyInternalIvk() {
	val ppkBytes = supp.getAsU8Array("account_public_key")
	val ivkBytes = supp.getAsU8Array("ppk_internal_ivk")

	val ppk = ZcashAccountPubKey(ppkBytes)
	val ivk = ppk.deriveInternalIvk()

	assert(ivk.toBytes() == ivkBytes)
}
testAccountPubKeyInternalIvk()

fun testAccountPubKeyForShielding() {
	val ppkBytes = supp.getAsU8Array("account_public_key")
	val intOvkBytes = supp.getAsU8Array("ppk_internal_ovk")
	val extOvkBytes = supp.getAsU8Array("ppk_external_ovk")

	val ppk = ZcashAccountPubKey(ppkBytes)
	val ovks = ppk.ovksForShielding()

	assert(ovks.internalOvk.asBytes() == intOvkBytes)
	assert(ovks.externalOvk.asBytes() == extOvkBytes)
}
testAccountPubKeyForShielding()

fun testAccountPubKeyInternalOvk() {
	val ppkBytes = supp.getAsU8Array("account_public_key")
	val ovkBytes = supp.getAsU8Array("ppk_internal_ovk")
	val ppk = ZcashAccountPubKey(ppkBytes)
	val ovk = ppk.internalOvk()

	assert(ovk.asBytes() == ovkBytes)
}
testAccountPubKeyInternalOvk()

fun testAccountPubKeyExternalOvk() {
	val ppkBytes = supp.getAsU8Array("account_public_key")
	val ovkBytes = supp.getAsU8Array("ppk_external_ovk")
	val ppk = ZcashAccountPubKey(ppkBytes)
	val ovk = ppk.externalOvk()


	assert(ovk.asBytes() == ovkBytes)
}
testAccountPubKeyExternalOvk()



fun testExternalIvkDefaultAddress() {
    val ppkBytes = supp.getAsU8Array("account_public_key")

	val ppk = ZcashAccountPubKey(ppkBytes)

    val defaultAddress = ppk.deriveExternalIvk().defaultAddress()

    val expectedAddress = supp.getAsString("external_ivk_default_address_address")
    val expectedIndex = supp.getAsU32("external_ivk_default_address_index")

    val params = ZcashConsensusParameters.MAIN_NETWORK

    assert(defaultAddress.transparentAddress.encode(params) == expectedAddress)
    assert(defaultAddress.index == expectedIndex)
}
testExternalIvkDefaultAddress()

fun testExternalIvkToBytes() {
    // covered in account_pub_key
}
testExternalIvkToBytes()

fun testExternalIvkFromBytes() {
    val ppkBytes = supp.getAsU8Array("account_public_key")

	val ppk = ZcashAccountPubKey(ppkBytes)

    val bytes = ppk.deriveExternalIvk().toBytes()

    assert(ZcashExternalIvk.fromBytes(bytes).toBytes() == bytes)
}
testExternalIvkFromBytes()

fun testInternalIvkDefaultAddress() {
    val ppkBytes = supp.getAsU8Array("account_public_key")

	val ppk = ZcashAccountPubKey(ppkBytes)

    val defaultAddress = ppk.deriveInternalIvk().defaultAddress()

    val expectedAddress = supp.getAsString("internal_ivk_default_address_address")
    val expectedIndex = supp.getAsU32("internal_ivk_default_address_index")

    val params = ZcashConsensusParameters.MAIN_NETWORK

    assert(defaultAddress.transparentAddress.encode(params) == expectedAddress)
    assert(defaultAddress.index == expectedIndex)
}
testInternalIvkDefaultAddress()

fun testInternalIvkToBytes() {
    // covered in account_pub_key
}
testInternalIvkToBytes()

fun testInternalIvkFromBytes() {
    val ppkBytes = supp.getAsU8Array("account_public_key")

	val ppk = ZcashAccountPubKey(ppkBytes)

    val bytes = ppk.deriveInternalIvk().toBytes()

    assert(ZcashExternalIvk.fromBytes(bytes).toBytes() == bytes)
}
testInternalIvkFromBytes()
fun testExternalOvkAsBytes() {
    // covered in account_pub_key
}
testExternalOvkAsBytes()

fun testInternalOvkAsBytes() {
    // covered in account_pub_key

}
testInternalOvkAsBytes()



fun setupNetwork() = ZcashConsensusParameters.TEST_NETWORK

fun testTransparentAddressFromPublicKey() {
    val network = setupNetwork()

    val encodedPublicKeyAddress = supp.getAsString("t_address_public_key")

    val parsedAsPublicKey = ZcashTransparentAddress.decode(network, encodedPublicKeyAddress)
    val addrBytes = parsedAsPublicKey.toBytes()

	val addr = ZcashTransparentAddress.fromPublicKey(addrBytes)

    assert(addr.toBytes() == addrBytes)
}

fun testTransparentAddressFromScript() {
    val network = setupNetwork()

    val encodedScriptAddress = supp.getAsString("t_address_script")
    val parsedAsScript = ZcashTransparentAddress.decode(network, encodedScriptAddress)
    val addrBytes = parsedAsScript.toBytes()

    val addr = ZcashTransparentAddress.fromScript(addrBytes)

    assert(addr.toBytes() == addrBytes)
}

fun testTransparentAddressPublicKeyEncodeAndDecode() {
    val network = setupNetwork()

    val encodedPublicKeyAddress = supp.getAsString("t_address_public_key")
    val parsedAsPublicKey = ZcashTransparentAddress.decode(network, encodedPublicKeyAddress)

    assert(parsedAsPublicKey.isPublicKey())
    assert(encodedPublicKeyAddress == parsedAsPublicKey.encode(network))
}
testTransparentAddressPublicKeyEncodeAndDecode()

fun testTransparentAddressScriptEncodeAndDecode() {
    val network = setupNetwork()

    val encodedScriptAddress = supp.getAsString("t_address_script")
    val parsedAsScript = ZcashTransparentAddress.decode(network, encodedScriptAddress)

    assert(parsedAsScript.isScript())
    assert(encodedScriptAddress == parsedAsScript.encode(network))
}
testTransparentAddressScriptEncodeAndDecode()



fun testMemoBytesNew() {
    val bytes = supp.getAsU8Array("memo_bytes")

    val memoData = supp.getAsU8Array("memo_data")

    val memoBytes = ZcashMemoBytes(bytes)

    assert(memoBytes.data() == memoData)

    var thrown = false;
    try {
        val memoBytesTooLong = supp.getAsU8Array("memo_bytes_too_long")
        val memoBytes = ZcashMemoBytes(memoBytesTooLong)
    } catch (e: ZcashException.ArrayLengthMismatch) {
        thrown = true;
    }
    assert(thrown)
}
testMemoBytesNew()

fun testMemoBytesEmpty() {
    val expected = supp.getAsU8Array("memo_empty")

    assert(ZcashMemoBytes.empty().data() == expected)
}
testMemoBytesEmpty()



fun testDiversifierNew() {
    val expected = supp.getAsU8Array("diversifier")

    val diversifier = ZcashDiversifier(expected)

    assert(diversifier.toBytes() == expected)
}
testDiversifierNew()



fun testDiversifiableFullViewingKeyFromBytes() {
	val expectedBytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(expectedBytes)

	assert(dfvk.toBytes() == expectedBytes)

}
testDiversifiableFullViewingKeyFromBytes()

fun testDiversifiableFullViewingKeyFvk() {
	val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val fvk = dfvk.fvk()

    val expected = supp.getAsU8Array("diversifiable_fvk_fvk")

	assert(fvk.toBytes() == expected)

}
testDiversifiableFullViewingKeyFvk()

fun testDiversifiableFullViewingKeyToNk() {
	val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

	val nk = dfvk.toNk(ZcashScope.EXTERNAL)

    val expected = supp.getAsU8Array("diversifiable_fvk_nk")

	assert(nk.toBytes() == expected)
}
testDiversifiableFullViewingKeyToNk()

fun testDiversifiableFullViewingKeyToIvk() {
	val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

	val ivk = dfvk.toIvk(ZcashScope.EXTERNAL)

    val expected = supp.getAsU8Array("diversifiable_fvk_ivk")

	assert(ivk.toRepr() == expected)
}
testDiversifiableFullViewingKeyToIvk()

fun testDiversifiableFullViewingKeyToOvk() {
	val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

	val ovk = dfvk.toOvk(ZcashScope.EXTERNAL)

    val expected = supp.getAsU8Array("diversifiable_fvk_ovk")

	assert(ovk.toBytes() == expected)
}
testDiversifiableFullViewingKeyToOvk()

fun testDiversifiableFullViewingKeyAddress() {
	val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val index = ZcashDiversifierIndex.fromU32(1u)

	val address = dfvk.address(index)!!

    val expected = supp.getAsString("diversifiable_fvk_address")

	assert(address.encode(ZcashConsensusParameters.MAIN_NETWORK) == expected)
}
testDiversifiableFullViewingKeyAddress()

fun testDiversifiableFullViewingKeyFindAddress() {
    val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val index = ZcashDiversifierIndex.fromU32(1u)

	val address = dfvk.findAddress(index)!!

    val expectedIndex = supp.getAsU8Array("dfvk_find_address_index")
    val expectedAddress = supp.getAsString("dfvk_find_address_address")

	assert(address.diversifierIndex.toBytes() == expectedIndex)
    assert(address.address.encode(ZcashConsensusParameters.MAIN_NETWORK) == expectedAddress)
}
testDiversifiableFullViewingKeyFindAddress()

fun testDiversifiableFullViewingKeyDefaultAddress() {
    val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val index = ZcashDiversifierIndex.fromU32(1u)

	val address = dfvk.defaultAddress()

    val expectedIndex = supp.getAsU8Array("dfvk_default_address_index")
    val expectedAddress = supp.getAsString("dfvk_default_address_address")

	assert(address.diversifierIndex.toBytes() == expectedIndex)
    assert(address.address.encode(ZcashConsensusParameters.MAIN_NETWORK) == expectedAddress)
}
testDiversifiableFullViewingKeyDefaultAddress()

fun testDiversifiableFullViewingKeyDiversifiedAddress() {
    val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val diversifier = ZcashDiversifier(supp.getAsU8Array("diversifier"))

	val address = dfvk.diversifiedAddress(diversifier)!!


    val expected = supp.getAsString("dfvk_diversified_address")

    assert(address.encode(ZcashConsensusParameters.MAIN_NETWORK) == expected)
}
testDiversifiableFullViewingKeyDiversifiedAddress()

fun testDiversifiableFullViewingKeyChangeAddress() {
    val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val index = ZcashDiversifierIndex.fromU32(1u)

	val address = dfvk.changeAddress()

    val expectedIndex = supp.getAsU8Array("dfvk_change_address_index")
    val expectedAddress = supp.getAsString("dfvk_change_address_address")

	assert(address.diversifierIndex.toBytes() == expectedIndex)
    assert(address.address.encode(ZcashConsensusParameters.MAIN_NETWORK) == expectedAddress)
}
testDiversifiableFullViewingKeyChangeAddress()

fun testDiversifiableFullViewingKeyDiversifiedChangeAddress() {
    val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val diversifier = ZcashDiversifier(supp.getAsU8Array("diversifier"))

	val address = dfvk.diversifiedChangeAddress(diversifier)!!

    val expected = supp.getAsString("dfvk_diversified_change_address")

    assert(address.encode(ZcashConsensusParameters.MAIN_NETWORK) == expected)
}
testDiversifiableFullViewingKeyDiversifiedChangeAddress()

fun testDiversifiableFullViewingKeyDecryptDiversifier() {
    val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val address = dfvk.defaultAddress()!!.address

    val decrypted = dfvk.decryptDiversifier(address)!!

    val expected = supp.getAsU8Array("dfvk_decrypt_diversifier")

    assert(decrypted.diversifierIndex.toBytes() == expected)
    assert(decrypted.scope == ZcashScope.EXTERNAL)
}
testDiversifiableFullViewingKeyDecryptDiversifier()



fun testExpandedSpendingKeyFromSpendingKey() {
    val bytes = supp.getAsU8Array("extended_spending_key")

	val key = ZcashExpandedSpendingKey.fromSpendingKey(bytes)

	val expected = supp.getAsU8Array("expanded_spending_key")

	assert(key.toBytes() == expected)
}
testExpandedSpendingKeyFromSpendingKey()

fun testExpandedSpendingKeyFromBytes() {
	val bytes = supp.getAsU8Array("expanded_spending_key")

	val key = ZcashExpandedSpendingKey.fromBytes(bytes)

	assert(key.toBytes() == bytes)
}
testExpandedSpendingKeyFromBytes()

fun testExpandedSpendingKeyProofGenerationKey() {
    // todo
}
testExpandedSpendingKeyProofGenerationKey()



fun testExtendedFullViewingKeyFromBytes() {
	val fvkBytes = supp.getAsU8Array("extended_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	assert(key.toBytes() == fvkBytes)
}
testExtendedFullViewingKeyFromBytes()

fun testExtendedFullViewingKeyEncodeAndDecode() {
	val network = ZcashConsensusParameters.MAIN_NETWORK

	val fvkAddr = supp.getAsString("extended_fvk_encoded")

	val decodedAddr = ZcashExtendedFullViewingKey.decode(network, fvkAddr)

	assert(decodedAddr.encode(network) == fvkAddr)
}
testExtendedFullViewingKeyEncodeAndDecode()

fun testExtendedFullViewingKeyDeriveChild() {
	val fvkBytes = supp.getAsU8Array("extended_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val index = ZcashChildIndex.NonHardened(32u)


	val efvkChild = key.deriveChild(index)

	val fvkChildBytes = supp.getAsU8Array("extended_fvk_child")

	assert(efvkChild.toBytes() == fvkChildBytes)
}
testExtendedFullViewingKeyDeriveChild()

fun testExtendedFullViewingKeyAddress() {
	val fvkBytes = supp.getAsU8Array("extended_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val divIdx = ZcashDiversifierIndex.fromU32(4u)

	val paymentAddress = key.address(divIdx)!!

	val fvkAddressBytes = supp.getAsU8Array("extended_fvk_address")

	assert(paymentAddress.toBytes() == fvkAddressBytes)
}
testExtendedFullViewingKeyAddress()

fun testExtendedFullViewingKeyFindAddress() {
	val fvkBytes = supp.getAsU8Array("extended_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val divIdx = ZcashDiversifierIndex.fromU32(0u)

	val paymentAddress = key.findAddress(divIdx)!!

    val expectedIndexBytes = supp.getAsU8Array("extended_fvk_find_address_index")
	val expectedAddressBytes = supp.getAsU8Array("extended_fvk_find_address_address")

    assert(paymentAddress.diversifierIndex.toBytes() == expectedIndexBytes)
	assert(paymentAddress.address.toBytes() == expectedAddressBytes)
}
testExtendedFullViewingKeyFindAddress()

fun testExtendedFullViewingKeyDefaultAddress() {
	val fvkBytes = supp.getAsU8Array("extended_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val paymentAddress = key.defaultAddress()

	val index = supp.getAsU8Array("extended_fvk_default_address_index")
    val address = supp.getAsU8Array("extended_fvk_default_address_address")

	assert(paymentAddress.diversifierIndex.toBytes() == index)
    assert(paymentAddress.address.toBytes() == address)
}
testExtendedFullViewingKeyDefaultAddress()

fun testExtendedFullViewingKeyDeriveInternal() {
	val fvkBytes = supp.getAsU8Array("extended_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val internalEfvk = key.deriveInternal()

	val efvkInternalBytes = supp.getAsU8Array("extended_fvk_derive_internal")

	assert(internalEfvk.toBytes() == efvkInternalBytes)
}
testExtendedFullViewingKeyDeriveInternal()

fun testExtendedFullViewingKeyToDiversifiableFvk() {
	val fvkBytes = supp.getAsU8Array("extended_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)


	val internalEfvk = key.toDiversifiableFullViewingKey()

	val efvkDivBytes = supp.getAsU8Array("extended_fvk_diversifiable_fvk")

	assert(internalEfvk.toBytes() == efvkDivBytes)
}
testExtendedFullViewingKeyToDiversifiableFvk()



fun testFullViewingKeyFromBytes() {
    val expectedBytes = supp.getAsU8Array("sapling_full_viewing_key")
    val fvk = ZcashFullViewingKey.fromBytes(expectedBytes)

    assert(fvk.toBytes() == expectedBytes)
}
testFullViewingKeyFromBytes()

fun testFullViewingKeyFromExpandedSpendingKey() {
    val bytes = supp.getAsU8Array("extended_spending_key")

	val key = ZcashExpandedSpendingKey.fromSpendingKey(bytes)

    val fvk = ZcashFullViewingKey.fromExpandedSpendingKey(key)

    val expectedBytes = supp.getAsU8Array("sapling_full_viewing_key")

    assert(fvk.toBytes() == expectedBytes)
}
testFullViewingKeyFromExpandedSpendingKey()

fun testFullViewingKeyVk() {
    // todo
}
testFullViewingKeyVk()

fun testFullViewingKeyOvk() {
    val bytes = supp.getAsU8Array("sapling_full_viewing_key")

    val ovk = ZcashFullViewingKey.fromBytes(bytes).ovk()

    val expected = supp.getAsU8Array("sapling_full_viewing_key_ovk")

    assert(ovk.toBytes() == expected)
}
testFullViewingKeyOvk()



fun testOutgoingViewingKeyFromBytes() {
	val expectedBytes = supp.getAsU8Array("sapling_outgoing_viewing_key")
	val ovk = ZcashOutgoingViewingKey.fromBytes(expectedBytes)

	assert(ovk.toBytes() == expectedBytes)
}
testOutgoingViewingKeyFromBytes()

fun testSaplingOvkToBytes() {
	val seed = supp.getAsU8Array("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expectedBytes = supp.getAsU8Array("sapling_outgoing_viewing_key")

    val ovkBytes = unifiedSpendingKey.toUnifiedFullViewingKey()
        .sapling()!!.toOvk(ZcashScope.EXTERNAL).toBytes()

    assert(ovkBytes == expectedBytes)
}
testSaplingOvkToBytes()


fun testProofGenerationKeyToViewingKey() {
    // todo
}
testProofGenerationKeyToViewingKey()



fun testViewingKeyIvk() {
    val bytes = supp.getAsU8Array("extended_spending_key")

	val esk = ZcashExpandedSpendingKey.fromSpendingKey(bytes)

    val pgk = esk.proofGenerationKey()

    val vk = pgk.toViewingKey()

    val ivk = vk.ivk()

    val expected = supp.getAsU8Array("viewing_key_ivk")

    assert(ivk.toRepr() == expected)
}
testViewingKeyIvk()

fun testViewingKeyToPaymentAddress() {
    val bytes = supp.getAsU8Array("extended_spending_key")

	val esk = ZcashExpandedSpendingKey.fromSpendingKey(bytes)

    val pgk = esk.proofGenerationKey()

    val vk = pgk.toViewingKey()

    val diversifierBytes = supp.getAsU8Array("diversifier")

    val diversifier = ZcashDiversifier(diversifierBytes)

    val address = vk.toPaymentAddress(diversifier)!!

    val expected = supp.getAsU8Array("viewing_key_payment_address")

    assert(address.toBytes() == expected)
}
testViewingKeyToPaymentAddress()



fun testPaymentAddressFromBytes() {
    val expected = supp.getAsU8Array("viewing_key_payment_address")

    val address = ZcashPaymentAddress.fromBytes(expected)

    assert(address.toBytes() == expected)
}
testPaymentAddressFromBytes()

fun testPaymentAddressDecode() {
    val expected = supp.getAsU8Array("viewing_key_payment_address")


    val address = ZcashPaymentAddress.fromBytes(expected)

    val encoded = address.encode(ZcashConsensusParameters.MAIN_NETWORK)

    val decoded = ZcashPaymentAddress.decode(ZcashConsensusParameters.MAIN_NETWORK, encoded)

    assert(decoded.toBytes() == expected)
}
testPaymentAddressDecode()

fun testPaymentAddressDiversifier() {
    val bytes = supp.getAsU8Array("viewing_key_payment_address")

    val address = ZcashPaymentAddress.fromBytes(bytes)

    val diversifier = address.diversifier()

    val expected = supp.getAsU8Array("diversifier")

    assert(diversifier.toBytes() == expected)
}
testPaymentAddressDiversifier()

fun testPaymentAddressPkD() {
    // todo
}
testPaymentAddressPkD()

fun testPaymentAddressCreateNote() {
    // todo
}
testPaymentAddressCreateNote()



fun testSaplingIvkToPaymentAddress() {
    val bytes = supp.getAsU8Array("extended_spending_key")

	val esk = ZcashExpandedSpendingKey.fromSpendingKey(bytes)

    val pgk = esk.proofGenerationKey()

    val vk = pgk.toViewingKey()

    val ivk = vk.ivk()

    val diversifierBytes = supp.getAsU8Array("diversifier")

    val diversifier = ZcashDiversifier(diversifierBytes)

    val address = ivk.toPaymentAddress(diversifier)!!

    val expected = supp.getAsU8Array("sapling_ivk_payment_address")

    assert(address.toBytes() == expected)
}
testSaplingIvkToPaymentAddress()

fun testSaplingIvkToRepr() {
    val bytes = supp.getAsU8Array("extended_spending_key")

	val esk = ZcashExpandedSpendingKey.fromSpendingKey(bytes)

    val pgk = esk.proofGenerationKey()

    val vk = pgk.toViewingKey()

    val ivk = vk.ivk()

    val expected = supp.getAsU8Array("viewing_key_ivk")

    assert(ivk.toRepr() == expected)
}
testSaplingIvkToRepr()


// todo after transaction PRs are merged


// todo after transaction PRs are merged



fun testDiversifierIndexNew() {
    val index = ZcashDiversifierIndex()

    val expected = supp.getAsU8Array("diversifier_index")

    assert(index.toBytes() == expected)
}
testDiversifierIndexNew()

fun testDiversifierIndexFromU32() {
    val index = ZcashDiversifierIndex.fromU32(0u)

    val expected = supp.getAsU8Array("diversifier_index")

    assert(index.toBytes() == expected)
}
testDiversifierIndexFromU32()

fun testDiversifierIndexFromU64() {
    val index = ZcashDiversifierIndex.fromU32(0U)

    val expected = supp.getAsU8Array("diversifier_index")

    assert(index.toBytes() == expected)
}
testDiversifierIndexFromU64()

fun testDiversifierIndexIncrement() {
    val index = ZcashDiversifierIndex.fromU32(0u)

    index.increment()

    val expected = supp.getAsU8Array("diversifier_index_incremented")

    assert(index.toBytes() == expected)
}
testDiversifierIndexIncrement()

fun testDiversifierIndexToU32() {
    val index = ZcashDiversifierIndex.fromU32(5u)

    assert(index.toU32() == 5u)
}
testDiversifierIndexToU32()



fun testExtendedSpendingKeyMaster() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashExtendedSpendingKey.master(seed)


    val expected = supp.getAsU8Array("extended_spending_key")

    assert(key.toBytes() == expected)
}
testExtendedSpendingKeyMaster()

fun testExtendedSpendingKeyFromBytes() {
    val bytes = supp.getAsU8Array("extended_spending_key")

    val key = ZcashExtendedSpendingKey.fromBytes(bytes)

    assert(key.toBytes() == bytes)
}
testExtendedSpendingKeyFromBytes()

fun testExtendedSpendingKeyFromPath() {
    val seed = supp.getAsU8Array("seed")

    val master = ZcashExtendedSpendingKey.master(seed)

    val key = ZcashExtendedSpendingKey.fromPath(master, listOf(ZcashChildIndex.NonHardened(0u)))

    val expected = supp.getAsU8Array("esk_from_path")

    assert(key.toBytes() == expected)
}
testExtendedSpendingKeyFromPath()

fun testExtendedSpendingKeyDecode() {
    val encoded = supp.getAsString("esk_encoded")

    val params = ZcashConsensusParameters.MAIN_NETWORK

    val key = ZcashExtendedSpendingKey.decode(params, encoded)

    assert(key.encode(params) == encoded)
}
testExtendedSpendingKeyDecode()

fun testExtendedSpendingKeyDeriveChild() {
    val seed = supp.getAsU8Array("seed")

    val master = ZcashExtendedSpendingKey.master(seed)

    val key = master.deriveChild(ZcashChildIndex.NonHardened(0u))

    val expected = supp.getAsU8Array("extended_spending_key_child")

    assert(key.toBytes() == expected)
}
testExtendedSpendingKeyDeriveChild()

fun testExtendedSpendingKeyDefaultAddress() {
    val seed = supp.getAsU8Array("seed")

    val master = ZcashExtendedSpendingKey.master(seed)

    val defaultAddress = master.defaultAddress()

    val expectedIndex = supp.getAsU8Array("esk_default_address_index")

    val expectedAddress = supp.getAsU8Array("esk_default_address_address")

    assert(defaultAddress.diversifierIndex.toBytes() == expectedIndex)
    assert(defaultAddress.address.toBytes() == expectedAddress)
}
testExtendedSpendingKeyDefaultAddress()

fun testExtendedSpendingKeyToDiversifiableFullViewingKey() {
    val seed = supp.getAsU8Array("seed")

    val master = ZcashExtendedSpendingKey.master(seed)

    val dfvk = master.toDiversifiableFullViewingKey()

    val expected = supp.getAsU8Array("esk_to_dfvk")

    assert(dfvk.toBytes() == expected)
}
testExtendedSpendingKeyToDiversifiableFullViewingKey()




class TransactionBuilderTest(supp: TestSupport) {

    val supp = supp

    fun testTransparentWithNonStandardFees(){
        val expectedTransactionBytes = supp.getAsU8Array(
            "transaction_non_standard_fee")
            
        val key = ZcashUnifiedSpendingKey.fromBytes(ZcashKeysEra.ORCHARD, supp.getAsU8Array("unified_spending_key"))
        val address = key.transparent().toAccountPubkey().deriveExternalIvk().deriveAddress(0u)

        val prevCoin = ZcashTxOut(ZcashAmount(200), address.script())

        val secretKey = key.transparent().deriveExternalSecretKey(0u)

        val builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820u))

        builder.addTransparentInput(secretKey, ZcashOutPoint(List(32) { 0u }, 1u),  prevCoin)
        builder.addTransparentOutput(address, ZcashAmount(200))

        val prover = ZcashLocalTxProver.withDefaultLocation()

        val feeRule = ZcashFeeRules.FixedNonStandard(0u)

        val result = builder.build(prover, feeRule)

        assert(result.transaction.toBytes() == expectedTransactionBytes)
    }

    fun testTransparentWithStandardFees(){
        val expectedTransactionBytes = supp.getAsU8Array(
            "transaction_standard_fee")

        val key = ZcashUnifiedSpendingKey.fromBytes(ZcashKeysEra.ORCHARD, supp.getAsU8Array("unified_spending_key"))

        val address = key.transparent().toAccountPubkey().deriveExternalIvk().deriveAddress(0u)

        val prevCoin = ZcashTxOut(ZcashAmount(1200), address.script())

        val secretKey = key.transparent().deriveExternalSecretKey(0u)

        val builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820u))

        builder.addTransparentInput(secretKey, ZcashOutPoint(List(32) { 0u }, 1u),  prevCoin)
        builder.addTransparentOutput(address, ZcashAmount(200))

        val prover = ZcashLocalTxProver.withDefaultLocation()

        val feeRule = ZcashFeeRules.FixedStandard

        val result = builder.build(prover, feeRule)

        assert(result.transaction.toBytes() == expectedTransactionBytes)
        
    }

    fun testTransparentWithZip317StandardFee(){
        val expectedTransactionBytes = supp.getAsU8Array(
            "transaction_zip317_standard_fee")

        val key = ZcashUnifiedSpendingKey.fromBytes(ZcashKeysEra.ORCHARD, supp.getAsU8Array("unified_spending_key"))

        val address = key.transparent().toAccountPubkey().deriveExternalIvk().deriveAddress(0u)

        val prevCoin = ZcashTxOut(ZcashAmount(19200), address.script())

        val secretKey = key.transparent().deriveExternalSecretKey(0u)

        val builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820u))

        builder.addTransparentInput(secretKey, ZcashOutPoint(List(32) { 0u }, 1u),  prevCoin)
        builder.addTransparentOutput(address, ZcashAmount(9200))

        val prover = ZcashLocalTxProver.withDefaultLocation()

        val feeRule = ZcashFeeRules.Zip317Standard

        val result = builder.build(prover, feeRule)

        assert(result.transaction.toBytes() == expectedTransactionBytes)
        
    }

    fun testTransparentWithZip317NonStandardFee(){
        val expectedTransactionBytes = supp.getAsU8Array(
            "transaction_zip317_non_standard_fee")

        val key = ZcashUnifiedSpendingKey.fromBytes(ZcashKeysEra.ORCHARD, supp.getAsU8Array("unified_spending_key"))

        val address = key.transparent().toAccountPubkey().deriveExternalIvk().deriveAddress(0u)

        val prevCoin = ZcashTxOut(ZcashAmount(19200), address.script())

        val secretKey = key.transparent().deriveExternalSecretKey(0u)

        val builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820u))

        builder.addTransparentInput(secretKey, ZcashOutPoint(List(32) { 0u }, 1u),  prevCoin)
        builder.addTransparentOutput(address, ZcashAmount(9200))

        val prover = ZcashLocalTxProver.withDefaultLocation()

        val feeRule = ZcashFeeRules.Zip317NonStandard(5000u, 2u, 150u, 34u)

        val result = builder.build(prover, feeRule)

        assert(result.transaction.toBytes() == expectedTransactionBytes)
        
    }

    fun testSaplingWithNonStandardFees(){
        val key = ZcashUnifiedSpendingKey.fromBytes(ZcashKeysEra.ORCHARD, supp.getAsU8Array("unified_spending_key"))

        val extsk = key.sapling()
        val paymentAddress = extsk.defaultAddress().address
        val rseed = ZcashRseed.AfterZip212(List(32) { 0u })
        val note = paymentAddress.createNote(200u, rseed)
        val tree = ZcashCommitmentTree.empty()
        tree.append(ZcashSaplingNode.fromCmu(note.cmu()))
        val witness = ZcashIncrementalWitness.fromTree(tree)

        val builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820u))

        builder.addSaplingSpend(extsk, paymentAddress.diversifier(), note, witness.path()!!)

        val ovk = key.sapling().toDiversifiableFullViewingKey().toOvk(ZcashScope.INTERNAL)
        builder.addSaplingOutput(
            ovk, paymentAddress, ZcashAmount(200), ZcashMemoBytes.empty())

        val prover = ZcashLocalTxProver.withDefaultLocation()
        val feeRule = ZcashFeeRules.FixedNonStandard(0u)

        val result = builder.build(prover, feeRule)
        // The output of each Sapling transaction differs each time.
        // This asserts the size, as its deterministic.
        assert(result.transaction.toBytes().size == 2377)
    }

    fun execute(){
        testTransparentWithNonStandardFees()
        testTransparentWithStandardFees()
        testTransparentWithZip317StandardFee()
        testTransparentWithZip317NonStandardFee()
        testSaplingWithNonStandardFees()
    }
    
}
TransactionBuilderTest(supp).execute()


class OrchardTransactionBuilderTest(supp: TestSupport) {

    val supp = supp

    fun testTransactionGeneration(){
        val key = ZcashUnifiedSpendingKey.fromBytes(ZcashKeysEra.ORCHARD, supp.getAsU8Array("unified_spending_key"))

        val ufvk = key.toUnifiedFullViewingKey()
        val fvk = ufvk.orchard()
        val ovk = fvk!!.toOvk(ZcashOrchardScope.EXTERNAL)
        val address = fvk!!.toIvk(ZcashOrchardScope.INTERNAL).address(ZcashOrchardDiversifier.fromBytes(List(11) { 0u }))

        // Note construction
        val noteValue = ZcashOrchardNoteValue.fromRaw(15u)
        val nullifier = ZcashOrchardNullifier.fromBytes(List(32) { 0u })
        val rseed = ZcashOrchardRandomSeed.fromBytes(List(32) { 0u }, nullifier)
        val note = ZcashOrchardNote.fromParts(address, noteValue, nullifier, rseed)

        val merkleHash = ZcashOrchardMerkleHash.fromBytes(List(32) { 0u })
        val authPath = List(32) { merkleHash }
        val merklePath = ZcashOrchardMerklePath.fromParts(0u, authPath)

        val anchor = merklePath.root(note.commitment().toExtractedNoteCommitment())
        val flags = ZcashOrchardFlags.fromParts(true, true)

        val builder = ZcashOrchardTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(
            2030820u), ZcashBlockHeight(2030820u+100u), anchor, flags)
        builder.addSpend(fvk!!, note, merklePath)
        builder.addRecipient(ovk, address, noteValue, null)

        val transaction = builder.build(listOf(key.orchard()), List(32) { 0u })

        assert(transaction.toBytes().size == 9165)

    }

    fun execute(){
       testTransactionGeneration()
    }
}
OrchardTransactionBuilderTest(supp).execute()

class TransactionSerializationTest(supp: TestSupport) {

    val supp = supp

    fun testTransactionFromBytes(){       
        val transactionBytes = supp.getAsU8Array("transaction_non_standard_fee")
        ZcashTransaction.fromBytes(transactionBytes, ZcashBranchId.NU5)
    }

    fun execute(){
        testTransactionFromBytes()
    }
}
TransactionSerializationTest(supp).execute()

class TransactionExplorationTest(supp: TestSupport) {

    val supp = supp

    fun testFirstLevelFields(){       
        val transactionBytes = supp.getAsU8Array("transaction_standard_fee")
        val tx = ZcashTransaction.fromBytes(transactionBytes, ZcashBranchId.NU5)

        // Id
        val idExpectedBytes = supp.getAsU8Array("transaction_standard_fee_id")
        assert(idExpectedBytes == tx.txid().toBytes())

        // Version
        val versionExpectedBytes = supp.getAsU8Array("transaction_standard_fee_version")
        assert(versionExpectedBytes == tx.version().toBytes())

        // lock time
        assert(0u == tx.lockTime())

        // expiry height
        assert(2030840u == tx.expiryHeight().value())
    }

    fun testTransparentBundle(){       
        val transactionBytes = supp.getAsU8Array("transaction_standard_fee")
        val tx = ZcashTransaction.fromBytes(transactionBytes, ZcashBranchId.NU5)

        val bundle = tx.transparentBundle()!!

        assert(bundle.isCoinbase() == false)

        // vout
        val vout = bundle.vout()

        assert(1 == vout.size)
        assert(200.toLong() == vout[0].value().value())

        val vout_0_bytes = supp.getAsU8Array("transaction_standard_fee_vout_0")
        assert(vout_0_bytes == vout[0].toBytes())

        val vout_0_address = supp.getAsU8Array("transaction_standard_fee_vout_0_recipient_address")
        assert(vout_0_address == vout[0].recipientAddress()!!.toBytes())

        val script_bytes = supp.getAsU8Array("transaction_standard_fee_vout_0_script")
        assert(script_bytes == vout[0].scriptPubkey().toBytes())

        // vin
        val vin = bundle.vin()

        assert(1 == vin.size)
        val vin0_bytes = supp.getAsU8Array("transaction_standard_fee_vin_0")
        assert(vin0_bytes == vin[0].toBytes())
    }

    fun testSaplingBundle(){       
        val transactionBytes = supp.getAsU8Array("transaction_sapling")
        val tx = ZcashTransaction.fromBytes(transactionBytes, ZcashBranchId.NU5)

        val bundle = tx.saplingBundle()!!

        // Shielded spends
        val spends = bundle.shieldedSpends()
        assert(1 == spends.size)
        val theSpend = spends[0]
        assert(supp.getAsU8Array("transaction_sapling_spend_0_cv") == theSpend.cv().toBytes())
        assert(supp.getAsU8Array("transaction_sapling_spend_0_anchor") == theSpend.anchor())
        assert(supp.getAsU8Array("transaction_sapling_spend_0_nullifier") == theSpend.nullifier().toBytes())
        assert(supp.getAsU8Array("transaction_sapling_spend_0_rk") == theSpend.rk().toBytes())

        // Shielded outputs
        val outputs = bundle.shieldedOutputs()
        assert(2 == outputs.size)
        val theOutput = outputs[0]
        assert(supp.getAsU8Array("transaction_sapling_output_0_cv") == theOutput.cv().toBytes())
        assert(supp.getAsU8Array("transaction_sapling_output_0_cmu") == theOutput.cmu().toBytes())

        // Value balance
        assert(0.toLong() == bundle.valueBalance().value())
    }

    fun testOrchardBundle(){       
        val transactionBytes = supp.getAsU8Array("transaction_orchard")
        val tx = ZcashTransaction.fromBytes(transactionBytes, ZcashBranchId.NU5)

        val bundle = tx.orchardBundle()!!

        // Actions
        val actions = bundle.actions()
        assert(2 == actions.size)

        val theAction = actions[1]

        assert(supp.getAsU8Array("transaction_orchard_action_1_nullifier") == theAction.nullifier().toBytes())
        assert(supp.getAsU8Array("transaction_orchard_action_1_cmx") == theAction.cmx().toBytes())
        assert(supp.getAsU8Array("transaction_orchard_action_1_encrypted_note_epk_bytes") == theAction.encryptedNote().epkBytes)
        assert(supp.getAsU8Array("transaction_orchard_action_1_encrypted_note_enc_ciphertext") == theAction.encryptedNote().encCiphertext)
        assert(supp.getAsU8Array("transaction_orchard_action_1_encrypted_note_out_ciphertext") == theAction.encryptedNote().outCiphertext)
        assert(supp.getAsU8Array("transaction_orchard_action_1_cv_net") == theAction.cvNet().toBytes())

        // Flags
        assert(supp.getAsU8Array("transaction_orchard_flags") == listOf(bundle.flags().toByte()))

        // Value balance
        assert(0.toLong() == bundle.valueBalance().value())

        // Anchor
        assert(supp.getAsU8Array("transaction_orchard_anchor") == bundle.anchor().toBytes())        
    }

    fun testOrchardBundleCrypto(){       
        
        val key = ZcashUnifiedSpendingKey.fromBytes(ZcashKeysEra.ORCHARD, supp.getAsU8Array("testnet_unified_spending_key"))

        val transactionBytes = supp.getAsU8Array("testnet_transaction_orchard")
        val tx = ZcashTransaction.fromBytes(transactionBytes, ZcashBranchId.NU5)

        val bundle = tx.orchardBundle()!!

        // // Verify proof
        // val verifyingKey = ZcashVerifyingKey()
        // bundle.verifyProof(verifyingKey)

        // Decrypt output with IVK
        val ivk = key.toUnifiedFullViewingKey().orchard()!!.toIvk(ZcashOrchardScope.INTERNAL)
        val output_ivk = bundle.decryptOutputWithKey(0u, ivk)
        assert(1999000.toULong() == output_ivk.note.value().value())
        assert(supp.getAsU8Array("testnet_transaction_orchard_address") == output_ivk.address.toRawAddressBytes())
        assert(supp.getAsU8Array("testnet_transaction_orchard_memo") == output_ivk.data)

        // Decrypt output with IVKs
        val outputs_ivk = bundle.decryptOutputWithKeys(listOf(ivk))
        assert(1 == outputs_ivk.size)
        val theOutput_ivk = outputs_ivk[0]
        assert(0.toULong() == theOutput_ivk.idx)
        assert(1999000.toULong() == theOutput_ivk.note.value().value())
        assert(ivk.toBytes() == theOutput_ivk.key.toBytes())
        assert(supp.getAsU8Array("testnet_transaction_orchard_address") == theOutput_ivk.address.toRawAddressBytes())
        assert(supp.getAsU8Array("testnet_transaction_orchard_memo") == theOutput_ivk.data)

        // Decrypt output with OVK
        val ovk = key.toUnifiedFullViewingKey().orchard()!!.toOvk(ZcashOrchardScope.INTERNAL)
        val output_ovk = bundle.recoverOutputWithOvk(0u, ovk)
        assert(1999000.toULong() == output_ovk.note.value().value())
        assert(supp.getAsU8Array("testnet_transaction_orchard_address") == output_ovk.address.toRawAddressBytes())
        assert(supp.getAsU8Array("testnet_transaction_orchard_memo") == output_ovk.data)

        // Decrypt output with OVKs
        val outputs_ovk = bundle.recoverOutputsWithOvks(listOf(ovk))
        assert(1 == outputs_ovk.size)
        val theOutput_ovk = outputs_ovk[0]
        assert(0.toULong() == theOutput_ovk.idx)
        assert(1999000.toULong() == theOutput_ovk.note.value().value())
        assert(ovk.toBytes() == theOutput_ovk.key.toBytes())
        assert(supp.getAsU8Array("testnet_transaction_orchard_address") == theOutput_ovk.address.toRawAddressBytes())
        assert(supp.getAsU8Array("testnet_transaction_orchard_memo") == theOutput_ovk.data)
    }

    fun execute(){
        testFirstLevelFields()
        testTransparentBundle()
        testSaplingBundle()
        testOrchardBundle()
        testOrchardBundleCrypto()
    }
}
TransactionExplorationTest(supp).execute()