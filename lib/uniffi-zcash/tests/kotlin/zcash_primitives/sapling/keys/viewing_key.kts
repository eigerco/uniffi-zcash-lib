import uniffi.zcash.*

fun testViewingKeyToPaymentAddress() {
    val supp = TestSupport.fromCsvFile()
    val fvkBytes = supp.getAsByteArray("sapling_full_viewing_key")
    val expectedBytes = supp.getAsByteArray("sapling_full_viewing_key_vk_payment_address")
    val saplingDiversifier = ZcashDiversifier(listOf(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() })
    val paymentAddress = ZcashFullViewingKey.fromBytes(fvkBytes).vk().toPaymentAddress(saplingDiversifier)!!

	assert(paymentAddress.toBytes() == expectedBytes)
}
testViewingKeyToPaymentAddress()

fun testViewingKeyIvk() {
    val supp = TestSupport.fromCsvFile()
    val fvkBytes = supp.getAsByteArray("sapling_full_viewing_key")
    val expectedBytes = supp.getAsByteArray("sapling_full_viewing_key_vk_ivk")
    val ivk = ZcashFullViewingKey.fromBytes(fvkBytes).vk().ivk()

	assert(ivk.toRepr() == expectedBytes)
}
testViewingKeyIvk()