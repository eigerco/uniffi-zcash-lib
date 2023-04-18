import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testSaplingIvkToPaymentAddress() {
    val bytes = supp.getAsU8Array("extended_spending_key")

	val esk = ZcashExpandedSpendingKey.fromSpendingKey(bytes)

    val pgk = esk.proofGenerationKey()

    val vk = pgk.toViewingKey()

    val ivk = vk.ivk()

    val diversifierBytes = supp.getAsU8Array("diversifier")

    val diversifier = ZcashDiversifier(diversifierBytes)

    val address = ivk.toPaymentAddress(diversifier)!!

    val expected = supp.getAsU8Array("sapling_ivk_payment_address")

    assert(address.toBytes() == expected)
}
testSaplingIvkToPaymentAddress()

fun testSaplingIvkToRepr() {
    val bytes = supp.getAsU8Array("extended_spending_key")

	val esk = ZcashExpandedSpendingKey.fromSpendingKey(bytes)

    val pgk = esk.proofGenerationKey()

    val vk = pgk.toViewingKey()

    val ivk = vk.ivk()

    val expected = supp.getAsU8Array("viewing_key_ivk")

    assert(ivk.toRepr() == expected)
}
testSaplingIvkToRepr()
