import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testRecipientAddressShielded() {
    val params = ZcashConsensusParameters.MAIN_NETWORK

    val source = supp.getAsString("recipient_address_shielded_source")

    val shielded = ZcashPaymentAddress.decode(params, source)

    val address = ZcashRecipientAddress.shielded(shielded)

    val expected = supp.getAsString("recipient_address_shielded")

    assert(address.encode(params) == expected)
}
testRecipientAddressShielded()

fun testRecipientAddressTransparent() {
    val params = ZcashConsensusParameters.MAIN_NETWORK

    val source = supp.getAsString("recipient_address_transparent_source")

    val transparent = ZcashTransparentAddress.decode(params, source)

    val address = ZcashRecipientAddress.transparent(transparent)

    val expected = supp.getAsString("recipient_address_transparent")

    assert(address.encode(params) == expected)
}
testRecipientAddressTransparent()

fun testRecipientAddressUnified() {
    val params = ZcashConsensusParameters.MAIN_NETWORK

    val source = supp.getAsString("recipient_address_unified_source")

    val unified = ZcashUnifiedAddress.decode(params, source)

    val address = ZcashRecipientAddress.unified(unified)

    val expected = supp.getAsString("recipient_address_unified")

    assert(address.encode(params) == expected)
}
testRecipientAddressUnified()

fun testRecipientAddressDecode() {
    val expected = supp.getAsString("recipient_address_unified")

    val params = ZcashConsensusParameters.MAIN_NETWORK

    val address = ZcashRecipientAddress.decode(params, expected)

    assert(address.encode(params) == expected)
}
testRecipientAddressDecode()

fun testRecipientAddressEncode() {
    // covered by testRecipientAddressDecode()
}
testRecipientAddressEncode()
