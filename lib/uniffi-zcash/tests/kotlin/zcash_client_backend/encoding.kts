import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testDecodeExtendedFullViewingKey() {
	val hrp = supp.getAsString("hrp_efvk")

	val fvk = supp.getAsString("extended_fvk_encoded")

	val decoded = decodeExtendedFullViewingKey(hrp, fvk)

	assert(encodeExtendedFullViewingKey(hrp, decoded) == fvk)
}
testDecodeExtendedFullViewingKey()

fun testDecodeExtendedSpendingKey() {
	val hrp = supp.getAsString("hrp_esk")

    val encoded = supp.getAsString("esk_encoded")

    val key = decodeExtendedSpendingKey(hrp, encoded)

    assert(encodeExtendedSpendingKey(hrp, key) == encoded)
}
testDecodeExtendedSpendingKey()

fun testDecodePaymentAddress() {
    val expected = supp.getAsU8Array("viewing_key_payment_address")

    val address = ZcashPaymentAddress.fromBytes(expected)

    val hrp = supp.getAsString("hrp_payment_address")

    val encoded = encodePaymentAddress(hrp, address)

    val decoded = decodePaymentAddress(hrp, encoded)

    assert(decoded.toBytes() == expected)
}
testDecodePaymentAddress()

fun testDecodeTransparentAddress() {
    val expected = supp.getAsString("t_address_script")

    val pubkey = supp.getAsU8Array("b58_pubkey_address_prefix")

    val script = supp.getAsU8Array("b58_script_address_prefix")

    val decoded = decodeTransparentAddress(pubkey, script, expected)

    val encoded = encodeTransparentAddress(pubkey, script, decoded)

    assert(encoded == expected)
}
testDecodeTransparentAddress()

fun testEncodeExtendedFullViewingKey() {
    // covered by testDecodeExtendedFullViewingKey()
}
testEncodeExtendedFullViewingKey()

fun testEncodeExtendedSpendingKey() {
    // covered by testDecodeExtendedSpendingKey()
}
testEncodeExtendedSpendingKey()

fun testEncodePaymentAddress() {
    // covered by testDecodePaymentAddress()
}
testEncodePaymentAddress()

fun testEncodePaymentAddressP() {
    val expected = supp.getAsU8Array("viewing_key_payment_address")

    val address = ZcashPaymentAddress.fromBytes(expected)

    val params = ZcashConsensusParameters.MAIN_NETWORK

    val encoded = encodePaymentAddressP(params, address)

    val hrp = supp.getAsString("hrp_payment_address")

    val decoded = decodePaymentAddress(hrp, encoded)

    assert(decoded.toBytes() == expected)
}
testEncodePaymentAddressP()

fun testEncodeTransparentAddress() {
    // covered by testDecodeTransparentAddress()
}
testEncodeTransparentAddress()

fun testEncodeTransparentAddressP() {
    val expected = supp.getAsString("t_address_script")

    val pubkey = supp.getAsU8Array("b58_pubkey_address_prefix")

    val script = supp.getAsU8Array("b58_script_address_prefix")

    val decoded = decodeTransparentAddress(pubkey, script, expected)

    val params = ZcashConsensusParameters.TEST_NETWORK

    val encoded = encodeTransparentAddressP(params, decoded)

    assert(encoded == expected)
}
testEncodeTransparentAddressP()
