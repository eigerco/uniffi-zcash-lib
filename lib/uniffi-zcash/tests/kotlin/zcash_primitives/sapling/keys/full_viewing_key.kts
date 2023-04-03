import uniffi.zcash.*

fun testFullViewingKeyFromBytes() {
    val supp = TestSupport.getFromCsv()
    val expectedBytes = supp.getAsByteArray("sapling_full_viewing_key")
    val fvk = ZcashFullViewingKey.fromBytes(expectedBytes)

    assert(fvk.toBytes() == expectedBytes)
}
testFullViewingKeyFromBytes()

fun testFullViewingKeyFromExpandedSpendingKey() {
    val supp = TestSupport.getFromCsv()
    val expectedBytes = supp.getAsByteArray("expanded_spending_key")
    val fvk = ZcashFullViewingKey.fromExpandedSpendingKey(expectedBytes)

    assert(fvk.toBytes() == expectedBytes)
}
testFullViewingKeyFromExpandedSpendingKey()

fun testFullViewingKeyVk() {
    val supp = TestSupport.getFromCsv()
    val expectedBytes = supp.getAsByteArray("sapling_full_viewing_key")
    val vkExpectedBytes = supp.getAsByteArray("sapling_full_viewing_key_vk")
    val vk = ZcashFullViewingKey.fromBytes(expectedBytes).vk()

    assert(vk.toBytes() == vkExpectedBytes)
}
testFullViewingKeyVk()

fun testFullViewingKeyOvk() {
    val supp = TestSupport.getFromCsv()
    val expectedBytes = supp.getAsByteArray("sapling_full_viewing_key")
    val ovkExpectedBytes = supp.getAsByteArray("sapling_full_viewing_key_ovk")
    val ovk = ZcashFullViewingKey.fromBytes(expectedBytes).ovk()

    assert(ovk.toBytes() == ovkExpectedBytes)
}
testFullViewingKeyOvk()