import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testDiversifierNew() {
    val expected = supp.getAsU8Array("diversifier")

    val diversifier = ZcashDiversifier(expected)

    assert(diversifier.toBytes() == expected)
}
testDiversifierNew()


fun testDiversifierToBytes() {
    // covered by testDiversifierNew()
}
testDiversifierToBytes()
