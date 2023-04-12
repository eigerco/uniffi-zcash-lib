import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testFullViewingKeyFromBytes() {
    val expectedBytes = supp.getAsU8Array("sapling_full_viewing_key")
    val fvk = ZcashFullViewingKey.fromBytes(expectedBytes)

    assert(fvk.toBytes() == expectedBytes)
}
testFullViewingKeyFromBytes()

fun testFullViewingKeyToBytes() {
    // covered by testFullViewingKeyToBytes()
}
testFullViewingKeyToBytes()

fun testFullViewingKeyFromExpandedSpendingKey() {
    val bytes = supp.getAsU8Array("extended_spending_key")

	val key = ZcashExpandedSpendingKey.fromSpendingKey(bytes)

    val fvk = ZcashFullViewingKey.fromExpandedSpendingKey(key)

    val expectedBytes = supp.getAsU8Array("sapling_full_viewing_key")

    assert(fvk.toBytes() == expectedBytes)
}
testFullViewingKeyFromExpandedSpendingKey()

fun testFullViewingKeyVk() {
    // todo
}
testFullViewingKeyVk()

fun testFullViewingKeyOvk() {
    val bytes = supp.getAsU8Array("sapling_full_viewing_key")

    val ovk = ZcashFullViewingKey.fromBytes(bytes).ovk()

    val expected = supp.getAsU8Array("sapling_full_viewing_key_ovk")

    assert(ovk.toBytes() == expected)
}
testFullViewingKeyOvk()
