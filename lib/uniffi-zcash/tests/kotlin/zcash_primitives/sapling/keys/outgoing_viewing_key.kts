import uniffi.zcash.*

fun testOutgoingViewingKeyFromBytes() {
	val supp = TestSupport.fromCsvFile()
	val expectedBytes = supp.getAsByteArray("sapling_outgoing_viewing_key")
	val ovk = ZcashOutgoingViewingKey.fromBytes(expectedBytes)

	assert(ovk.toBytes() == expectedBytes)
}
testOutgoingViewingKeyFromBytes()

fun testSaplingOvkToBytes() {
	val supp = TestSupport.fromCsvFile()
	val seed = supp.getAsByteArray("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expectedBytes = supp.getAsByteArray("sapling_outgoing_viewing_key")

    val ovkBytes = unifiedSpendingKey.toUnifiedFullViewingKey()
        .sapling()!!.toOvk(ZcashScope.EXTERNAL).toBytes()

    assert(ovkBytes == expectedBytes)
}
testSaplingOvkToBytes()