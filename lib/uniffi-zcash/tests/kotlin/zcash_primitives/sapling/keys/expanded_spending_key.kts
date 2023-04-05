import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testFromExpandedSpendingKey() {
	// is this the correct spending key?
	val skBytes = supp.getAsByteArray("extended_spending_key")

	val spendingKey = ZcashExpandedSpendingKey.fromSpendingKey(skBytes)

	assert(spendingKey.toBytes() == skBytes)
}
testFromExpandedSpendingKey()

fun testFromBytes() {
	val skBytes = supp.getAsByteArray("expanded_spending_key")

	val spendingKey = ZcashExpandedSpendingKey.fromBytes(skBytes)

	assert(spendingKey.toBytes() == skBytes)
}
testFromBytes()

fun testProofGenerationKey() {
	val skBytes = supp.getAsByteArray("expanded_spending_key")

	val spendingKey = ZcashExpandedSpendingKey.fromBytes(skBytes)

	val proofGenKey = spendingKey.proofGenerationKey()

	// TODO finish this, but how?
	assert(false)

}
testProofGenerationKey()
