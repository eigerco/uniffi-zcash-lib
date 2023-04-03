import uniffi.zcash.*

fun testFromExpandedSpendingKey() {
	val supp = TestSupport.fromCsvFile()

	// is this the correct spending key?
	val skBytes = supp.getAsByteArray("extended_spending_key")

	val spendingKey = ZcashExpandedSpendingKey.fromSpendingKey(skBytes)

	assert(spendingKey.toBytes() == skBytes)
}
testFromExpandedSpendingKey()

fun testFromBytes() {
	val supp = TestSupport.fromCsvFile()

	val skBytes = supp.getAsByteArray("expanded_spending_key")

	val spendingKey = ZcashExpandedSpendingKey.fromBytes(skBytes)

	assert(spendingKey.toBytes() == skBytes)
}
testFromBytes()

fun testProofGenerationKey() {
	val supp = TestSupport.fromCsvFile()

	val skBytes = supp.getAsByteArray("expanded_spending_key")

	val spendingKey = ZcashExpandedSpendingKey.fromBytes(skBytes)

	val proofGenKey = spendingKey.proofGenerationKey()

	// TODO finish this, but how?
	assert(false)

}
testProofGenerationKey()
