import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testPaymentAddressFromBytes() {
    val expected = supp.getAsU8Array("viewing_key_payment_address")

    val address = ZcashPaymentAddress.fromBytes(expected)

    assert(address.toBytes() == expected)
}
testPaymentAddressFromBytes()

fun testPaymentAddressDecode() {
    val expected = supp.getAsU8Array("viewing_key_payment_address")

    val address = ZcashPaymentAddress.fromBytes(expected)

    val encoded = address.encode(ZcashConsensusParameters.MAIN_NETWORK)

    val decoded = ZcashPaymentAddress.decode(ZcashConsensusParameters.MAIN_NETWORK, encoded)

    assert(decoded.toBytes() == expected)
}
testPaymentAddressDecode()

fun testPaymentAddressDiversifier() {
    val bytes = supp.getAsU8Array("viewing_key_payment_address")

    val address = ZcashPaymentAddress.fromBytes(bytes)

    val diversifier = address.diversifier()

    val expected = supp.getAsU8Array("diversifier")

    assert(diversifier.toBytes() == expected)
}
testPaymentAddressDiversifier()

fun testPaymentAddressPkD() {
    // todo
}
testPaymentAddressPkD()

fun testPaymentAddressCreateNote() {
    // todo
}
testPaymentAddressCreateNote()
