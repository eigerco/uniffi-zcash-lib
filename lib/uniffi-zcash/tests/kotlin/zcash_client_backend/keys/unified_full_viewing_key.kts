import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

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
    val seed = supp.getAsByteArray("seed")
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

fun testUnifiedFullViewingKeyTransparent() {
    // covered by testUnifiedFullViewingKeyNew()
}
testUnifiedFullViewingKeyTransparent()

fun testUnifiedFullViewingKeySapling() {
    // covered by testUnifiedFullViewingKeyNew()
}
testUnifiedFullViewingKeySapling()

fun testUnifiedFullViewingKeyOrchard() {
    // covered by testUnifiedFullViewingKeyNew()
}
testUnifiedFullViewingKeyOrchard()

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

    val expectedIndex = supp.getAsByteArray("unified_full_viewing_key_find_address_index");

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

    val expectedIndex = supp.getAsByteArray("unified_full_viewing_key_default_address_index");

    assert(defaultAddress.address.encode(params) == expectedAddress)
    assert(defaultAddress.diversifierIndex.toBytes() == expectedIndex)
}
testUnifiedFullViewingKeyDefaultAddress()
