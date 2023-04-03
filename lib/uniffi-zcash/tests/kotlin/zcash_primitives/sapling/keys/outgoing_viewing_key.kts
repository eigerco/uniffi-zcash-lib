import uniffi.zcash.*

fun testOutgoingViewingKeyFromBytes() {
	val supp = TestSupport.fromCsvFile()
	val expectedBytes = supp.getAsByteArray("sapling_outgoing_viewing_key")
	val ovk = ZcashOutgoingViewingKey.fromBytes(expectedBytes)

	assert(ovk.toBytes() == expectedBytes)
}
testOutgoingViewingKeyFromBytes()