import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testViewingKeyIvk() {
    val bytes = supp.getAsU8Array("extended_spending_key")

	val esk = ZcashExpandedSpendingKey.fromSpendingKey(bytes)

    val pgk = esk.proofGenerationKey()

    val vk = pgk.toViewingKey()

    val ivk = vk.ivk()

    val expected = supp.getAsU8Array("viewing_key_ivk")

    assert(ivk.toRepr() == expected)
}
testViewingKeyIvk()

fun testViewingKeyToPaymentAddress() {
    val bytes = supp.getAsU8Array("extended_spending_key")

	val esk = ZcashExpandedSpendingKey.fromSpendingKey(bytes)

    val pgk = esk.proofGenerationKey()

    val vk = pgk.toViewingKey()

    val diversifierBytes = supp.getAsU8Array("diversifier")

    val diversifier = ZcashDiversifier(diversifierBytes)

    val address = vk.toPaymentAddress(diversifier)!!

    val expected = supp.getAsU8Array("viewing_key_payment_address")

    assert(address.toBytes() == expected)
}
testViewingKeyToPaymentAddress()
