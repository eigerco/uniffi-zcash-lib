import uniffi.zcash.*

fun testTransparentAddressFromPublicKey() {
    val network = ZcashConsensusParameters.TEST_NETWORK
    val supp = TestSupport.fromCsvFile()

    val encodedPublicKeyAddress = supp.getAsString("t_address_public_key")
    val parsedAsPublicKey = ZcashTransparentAddress.decode(network, encodedPublicKeyAddress)
    val addrBytes = parsedAsPublicKey.toBytes()

	val addr = ZcashTransparentAddress.publicKey(addrBytes)

    assert(addr.toBytes() == addrBytes)
}

fun testTransparentAddressFromScript() {
    val network = ZcashConsensusParameters.TEST_NETWORK
    val supp = TestSupport.fromCsvFile()

    val encodedScriptAddress = supp.getAsString("t_address_script")
    val parsedAsScript = ZcashTransparentAddress.decode(network, encodedScriptAddress)
    val addrBytes = parsedAsScript.toBytes()

    val addr = ZcashTransparentAddress.script(addrBytes)

    assert(addr.toBytes() == addrBytes)
}

fun testTransparentAddressPublicKeyEncodeAndDecode() {
    val network = ZcashConsensusParameters.TEST_NETWORK
    val supp = TestSupport.fromCsvFile()

    val encodedPublicKeyAddress = supp.getAsString("t_address_public_key")
    val parsedAsPublicKey = ZcashTransparentAddress.decode(network, encodedPublicKeyAddress)

    assert(parsedAsPublicKey.isPublicKey())
    assert(encodedPublicKeyAddress == parsedAsPublicKey.encode(network))
}
testTransparentAddressPublicKeyEncodeAndDecode()


fun testTransparentAddressScriptEncodeAndDecode() {
    val network = ZcashConsensusParameters.TEST_NETWORK
    val supp = TestSupport.fromCsvFile()

    val encodedScriptAddress = supp.getAsString("t_address_script")
    val parsedAsScript = ZcashTransparentAddress.decode(network, encodedScriptAddress)

    assert(parsedAsScript.isScript())
    assert(encodedScriptAddress == parsedAsScript.encode(network))
}
testTransparentAddressScriptEncodeAndDecode()

