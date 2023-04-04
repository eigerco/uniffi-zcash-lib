import uniffi.zcash.*

fun testViewingKeyToPaymentAddress() {
    val supp = TestSupport.getFromCsv()
    val fvkBytes = supp.getAsByteArray("sapling_full_viewing_key")
    val expectedBytes = supp.getAsByteArray("sapling_full_viewing_key_vk_payment_address")
    val vk = ZcashFullViewingKey.fromBytes(fvkBytes).vk()

	assert(vk.toPaymentAddress().toBytes() == expectedBytes)
}
testViewingKeyToPaymentAddress()

fun testViewingKeyIvk() {
    val supp = TestSupport.getFromCsv()
    val fvkBytes = supp.getAsByteArray("sapling_full_viewing_key")
    val expectedBytes = supp.getAsByteArray("sapling_full_viewing_key_vk_ivk")
    val vk = ZcashFullViewingKey.fromBytes(fvkBytes).vk()

	assert(vk.ivk().toBytes() == expectedBytes)
}
testViewingKeyIvk()