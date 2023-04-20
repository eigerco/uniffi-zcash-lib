import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testAllGetters() {
	assert(supp.getAsU8Array("seed") == listOf(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() })
	assert(supp.getAsU32Array("seed") == listOf(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUInt() })
    assert(supp.getAsU64Array("seed") == listOf(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toULong() })
	assert(supp.getAsU32("coin_type") == 234u)
    assert(supp.getAsU64("coin_type") == 234.toULong())
	assert(supp.getAsString("test_string") == "TestString")
}
testAllGetters()
