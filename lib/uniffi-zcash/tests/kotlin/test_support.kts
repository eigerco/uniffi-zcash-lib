import uniffi.zcash.*

fun testAllGetters() {
	val supp = TestSupport.fromCsvFile()
	val seed = listOf(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() }
	val seedUInt = listOf(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUInt() }

	assert(supp.getAsByteArray("seed") == seed)
	assert(supp.getAsIntegerArray("seed") == seedUInt)
	assert(supp.getAsInteger("coin_type") == 234u)
	assert(supp.getAsString("scope") == "External")

}
testAllGetters()