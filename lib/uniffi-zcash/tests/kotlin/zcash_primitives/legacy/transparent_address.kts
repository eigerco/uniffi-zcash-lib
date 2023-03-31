import uniffi.zcash.*

fun testTransparentAddressFromPublicKey() {
	ZcashTransparentAddress.publicKey()
}

fun testTransparentAddressFromScript() {

}

fun testTransparentAddressEncode() {

}

fun testTransparentAddressParsingEncodeAndDecode() {
    val net = ZcashConsensusParameters.TEST_NETWORK
    val input = "tm9iMLAuYMzJ6jtFLcA7rzUmfreGuKvr7Ma"
    val parsed = ZcashTransparentAddress.decode(net, input)

    assert(parsed.isPublicKey())
    assert(input == parsed.encode(net))

    val input2 = "t26YoyZ1iPgiMEWL4zGUm74eVWfhyDMXzY2"
    val parsed2 = ZcashTransparentAddress.decode(net, input2)

    assert(parsed2.isScript())
    assert(input2 == parsed2.encode(net))
}
testTransparentAddressParsingEncodeAndDecode()

