import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testExtendedSpendingKeyMaster() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val expected = supp.getAsU8Array("extended_spending_key")

    assert(key.toBytes() == expected)
}
testExtendedSpendingKeyMaster()

fun testExtendedSpendingKeyFromBytes() {
    val bytes = supp.getAsU8Array("extended_spending_key")

    val key = ZcashExtendedSpendingKey.fromBytes(bytes)

    assert(key.toBytes() == bytes)
}
testExtendedSpendingKeyFromBytes()

fun testExtendedSpendingKeyFromPath() {
    val seed = supp.getAsU8Array("seed")

    val master = ZcashExtendedSpendingKey.master(seed)

    val key = ZcashExtendedSpendingKey.fromPath(master, listOf(ZcashChildIndex.NonHardened(0u)))

    val expected = supp.getAsU8Array("esk_from_path")

    assert(key.toBytes() == expected)
}
testExtendedSpendingKeyFromPath()

fun testExtendedSpendingKeyDecode() {
    val encoded = supp.getAsString("esk_encoded")

    val params = ZcashConsensusParameters.MAIN_NETWORK

    val key = ZcashExtendedSpendingKey.decode(params, encoded)

    assert(key.encode(params) == encoded)
}
testExtendedSpendingKeyDecode()

fun testExtendedSpendingKeyEncode() {
    // covered by testExtendedSpendingKeyDecode()
}
testExtendedSpendingKeyEncode()

fun testExtendedSpendingKeyToBytes() {
    // covered by testExtendedSpendingKeyFromBytes()
}
testExtendedSpendingKeyToBytes()

fun testExtendedSpendingKeyDeriveChild() {
    val seed = supp.getAsU8Array("seed")

    val master = ZcashExtendedSpendingKey.master(seed)

    val key = master.deriveChild(ZcashChildIndex.NonHardened(0u))

    val expected = supp.getAsU8Array("extended_spending_key_child")

    assert(key.toBytes() == expected)
}
testExtendedSpendingKeyDeriveChild()

fun testExtendedSpendingKeyDefaultAddress() {
    val seed = supp.getAsU8Array("seed")

    val master = ZcashExtendedSpendingKey.master(seed)

    val defaultAddress = master.defaultAddress()

    val expectedIndex = supp.getAsU8Array("esk_default_address_index")

    val expectedAddress = supp.getAsU8Array("esk_default_address_address")

    assert(defaultAddress.diversifierIndex.toBytes() == expectedIndex)
    assert(defaultAddress.address.toBytes() == expectedAddress)
}
testExtendedSpendingKeyDefaultAddress()

fun testExtendedSpendingKeyToDiversifiableFullViewingKey() {
    val seed = supp.getAsU8Array("seed")

    val master = ZcashExtendedSpendingKey.master(seed)

    val dfvk = master.toDiversifiableFullViewingKey()

    val expected = supp.getAsU8Array("esk_to_dfvk")

    assert(dfvk.toBytes() == expected)
}
testExtendedSpendingKeyToDiversifiableFullViewingKey()
