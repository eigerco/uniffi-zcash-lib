import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testDiversifiableFullViewingKeyFromBytes() {
	val expectedBytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(expectedBytes)

	assert(dfvk.toBytes() == expectedBytes)

}
testDiversifiableFullViewingKeyFromBytes()

fun testDiversifiableFullViewingKeyToBytes() {
    // covered by testDiversifiableFullViewingKeyFromBytes()
}
testDiversifiableFullViewingKeyToBytes()

fun testDiversifiableFullViewingKeyFvk() {
	val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(expectedBytesDiversifiable)
	val fvk = dfvk.fvk()

    val expected = supp.getAsU8Array("diversifiable_fvk_fvk")

	assert(fvk.toBytes() == expected)

}
testDiversifiableFullViewingKeyFvk()

fun testDiversifiableFullViewingKeyToNk() {
	val expectedBytes = supp.getAsU8Array("diversifiable_fvk")
	val expectedNkBytes = supp.getAsU8Array("extended_spending_key_fvk_nk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(expectedBytes)
	val nk = dfvk.toNk(ZcashScope.EXTERNAL)

	assert(nk.toBytes() == expectedNkBytes)
}
testDiversifiableFullViewingKeyToNk()

fun testDiversifiableFullViewingKeyToIvk() {
	val expectedBytes = supp.getAsU8Array("diversifiable_fvk")
	val expectedIvkBytes = supp.getAsU8Array("extended_spending_key_fvk_ivk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(expectedBytes)
	val ivk = dfvk.toIvk(ZcashScope.EXTERNAL)

	assert(ivk.toRepr() == expectedIvkBytes)
}
testDiversifiableFullViewingKeyToIvk()

fun testDiversifiableFullViewingKeyToOvk() {
	val expectedBytes = supp.getAsU8Array("diversifiable_fvk")
	val expectedOvkBytes = supp.getAsU8Array("extended_spending_key_fvk_ovk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(expectedBytes)
	val ovk = dfvk.toOvk(ZcashScope.EXTERNAL)

	assert(ovk.toBytes() == expectedOvkBytes)
}
testDiversifiableFullViewingKeyToOvk()

fun testDiversifiableFullViewingKeyAddress() {
	val expectedBytes = supp.getAsU8Array("diversifiable_fvk")
	val expectedAddrBytes = supp.getAsU8Array("extended_spending_key_fvk_addr")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(expectedBytes)
	val diversifier = ZcashDiversifier(listOf(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() })
	val addr = dfvk.address(diversifier)!!

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
