import uniffi.zcash.*

fun testDiversifiableFullViewingKeyFromBytes() {
	val supp = TestSupport.fromCsvFile()
	val expectedBytes = supp.getAsByteArray("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(expectedBytes)

	assert(dfvk.toBytes() == expectedBytes)

}
testDiversifiableFullViewingKeyFromBytes()

fun testDiversifiableFullViewingKeyFvk() {
	val supp = TestSupport.fromCsvFile()
	val expectedBytesDiversifiable = supp.getAsByteArray("diversifiable_fvk")
	val expectedBytes = supp.getAsByteArray("extended_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(expectedBytesDiversifiable)
	val fvk = dfvk.fvk()
	// TODO something wrong here?

	assert(dfvk.toBytes() == fvk.toBytes())

}
testDiversifiableFullViewingKeyFvk()

fun testDiversifiableFullViewingKeyToNk() {
	val supp = TestSupport.fromCsvFile()
	val expectedBytes = supp.getAsByteArray("diversifiable_fvk")
	val expectedNkBytes = supp.getAsByteArray("extended_spending_key_fvk_nk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(expectedBytes)
	val nk = dfvk.toNk(ZcashScope.EXTERNAL)

	assert(nk.toBytes() == expectedNkBytes)
}
testDiversifiableFullViewingKeyToNk()

fun testDiversifiableFullViewingKeyToIvk() {
	val supp = TestSupport.fromCsvFile()
	val expectedBytes = supp.getAsByteArray("diversifiable_fvk")
	val expectedIvkBytes = supp.getAsByteArray("extended_spending_key_fvk_ivk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(expectedBytes)
	val ivk = dfvk.toIvk(ZcashScope.EXTERNAL)

	assert(ivk.toBytes() == expectedIvkBytes)
}
testDiversifiableFullViewingKeyToIvk()

fun testDiversifiableFullViewingKeyToOvk() {
	val supp = TestSupport.fromCsvFile()
	val expectedBytes = supp.getAsByteArray("diversifiable_fvk")
	val expectedOvkBytes = supp.getAsByteArray("extended_spending_key_fvk_ovk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(expectedBytes)
	val ovk = dfvk.toOvk(ZcashScope.EXTERNAL)

	assert(ovk.toBytes() == expectedOvkBytes)
}
testDiversifiableFullViewingKeyToOvk()

fun testDiversifiableFullViewingKeyAddress() {
	val supp = TestSupport.fromCsvFile()
	val expectedBytes = supp.getAsByteArray("diversifiable_fvk")
	val expectedAddrBytes = supp.getAsByteArray("extended_spending_key_fvk_addr")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(expectedBytes)
	val diversifier = ZcashDiversifier.new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
	val addr = dfvk.address(diversifier)

	assert(addr.toBytes() == expectedAddrBytes)
}
testDiversifiableFullViewingKeyAddress()

// TODO
// find_address
// default_address
// diversified_address
// change_address
// diversified_change_address
// decrypt_diversifier