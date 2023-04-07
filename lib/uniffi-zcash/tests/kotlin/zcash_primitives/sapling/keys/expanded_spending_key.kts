import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testFromExpandedSpendingKey() {
	// is this the correct spending key?
	val skBytes = supp.getAsU8Array("extended_spending_key")

	val spendingKey = ZcashExpandedSpendingKey.fromSpendingKey(skBytes)

	assert(spendingKey.toBytes() == skBytes)
}
testFromExpandedSpendingKey()

fun testFromBytes() {
	val skBytes = supp.getAsU8Array("expanded_spending_key")

	val spendingKey = ZcashExpandedSpendingKey.fromBytes(skBytes)

	assert(spendingKey.toBytes() == skBytes)
}
testFromBytes()

fun testProofGenerationKey() {
	val skBytes = supp.getAsU8Array("expanded_spending_key")

	val spendingKey = ZcashExpandedSpendingKey.fromBytes(skBytes)

	val proofGenKey = spendingKey.proofGenerationKey()

	// TODO finish this, but how?
	assert(false)

}
testProofGenerationKey()
