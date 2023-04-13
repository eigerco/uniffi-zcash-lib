import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

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
