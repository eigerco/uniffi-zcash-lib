import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

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

fun testExpandedSpendingKeyToBytes() {
    // covered by testExpandedSpendingKeyFromBytes()
}
testExpandedSpendingKeyToBytes()
