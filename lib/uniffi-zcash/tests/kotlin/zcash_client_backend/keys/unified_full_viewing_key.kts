import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

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
