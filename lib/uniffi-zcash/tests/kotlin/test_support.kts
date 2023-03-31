import uniffi.zcash.*

fun testAllGetters() {
	val supp = TestSupport.fromCsvFile()

	assert(supp.getAsByteArray("seed") == [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
	assert(supp.getAsIntegerArray("seed") == [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
	assert(supp.getAsStringArray("extended_spending_key_child_index") == ["1", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0"])
	assert(supp.getAsInteger("coin_type") == 234)
	assert(supp.getAsString("scope") == "External")

}
testAllGetters()