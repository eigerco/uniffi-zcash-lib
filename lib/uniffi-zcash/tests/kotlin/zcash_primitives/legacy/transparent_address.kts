import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun setupNetwork() = ZcashConsensusParameters.TEST_NETWORK

fun testTransparentAddressFromPublicKey() {
    val network = setupNetwork()

    val encodedPublicKeyAddress = supp.getAsString("t_address_public_key")
    val parsedAsPublicKey = ZcashTransparentAddress.decode(network, encodedPublicKeyAddress)
    val addrBytes = parsedAsPublicKey.toBytes()

	val addr = ZcashTransparentAddress.fromPublicKey(addrBytes)

    assert(addr.toBytes() == addrBytes)
}

fun testTransparentAddressFromScript() {
    val network = setupNetwork()

    val encodedScriptAddress = supp.getAsString("t_address_script")
    val parsedAsScript = ZcashTransparentAddress.decode(network, encodedScriptAddress)
    val addrBytes = parsedAsScript.toBytes()

    val addr = ZcashTransparentAddress.fromScript(addrBytes)

    assert(addr.toBytes() == addrBytes)
}

fun testTransparentAddressPublicKeyEncodeAndDecode() {
    val network = setupNetwork()

    val encodedPublicKeyAddress = supp.getAsString("t_address_public_key")
    val parsedAsPublicKey = ZcashTransparentAddress.decode(network, encodedPublicKeyAddress)

    assert(parsedAsPublicKey.isPublicKey())
    assert(encodedPublicKeyAddress == parsedAsPublicKey.encode(network))
}
testTransparentAddressPublicKeyEncodeAndDecode()

fun testTransparentAddressScriptEncodeAndDecode() {
    val network = setupNetwork()

    val encodedScriptAddress = supp.getAsString("t_address_script")
    val parsedAsScript = ZcashTransparentAddress.decode(network, encodedScriptAddress)

    assert(parsedAsScript.isScript())
    assert(encodedScriptAddress == parsedAsScript.encode(network))
}
testTransparentAddressScriptEncodeAndDecode()

fun testTransparentAddressIsPublicKey() {
    // covered by testTransparentAddressPublicKeyEncodeAndDecode()
}
testTransparentAddressIsPublicKey()

fun testTransparentAddresIsScript() {
    // covered by testTransparentAddressScriptEncodeAndDecode()
}
testTransparentAddresIsScript()
