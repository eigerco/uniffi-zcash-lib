import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testExternalIvkDefaultAddress() {
    val ppkBytes = supp.getAsU8Array("account_public_key")

	val ppk = ZcashAccountPubKey(ppkBytes)

    val defaultAddress = ppk.deriveExternalIvk().defaultAddress()

    val expectedAddress = supp.getAsString("external_ivk_default_address_address")
    val expectedIndex = supp.getAsU32("external_ivk_default_address_index")

    val params = ZcashConsensusParameters.MAIN_NETWORK

    assert(defaultAddress.transparentAddress.encode(params) == expectedAddress)
    assert(defaultAddress.index == expectedIndex)
}
testExternalIvkDefaultAddress()

fun testExternalIvkToBytes() {
    // covered in account_pub_key
}
testExternalIvkToBytes()

fun testExternalIvkFromBytes() {
    val ppkBytes = supp.getAsU8Array("account_public_key")

	val ppk = ZcashAccountPubKey(ppkBytes)

    val bytes = ppk.deriveExternalIvk().toBytes()

    assert(ZcashExternalIvk.fromBytes(bytes).toBytes() == bytes)
}
testExternalIvkFromBytes()

fun testInternalIvkDefaultAddress() {
    val ppkBytes = supp.getAsU8Array("account_public_key")

	val ppk = ZcashAccountPubKey(ppkBytes)

    val defaultAddress = ppk.deriveInternalIvk().defaultAddress()

    val expectedAddress = supp.getAsString("internal_ivk_default_address_address")
    val expectedIndex = supp.getAsU32("internal_ivk_default_address_index")

    val params = ZcashConsensusParameters.MAIN_NETWORK

    assert(defaultAddress.transparentAddress.encode(params) == expectedAddress)
    assert(defaultAddress.index == expectedIndex)
}
testInternalIvkDefaultAddress()

fun testInternalIvkToBytes() {
    // covered in account_pub_key
}
testInternalIvkToBytes()

fun testInternalIvkFromBytes() {
    val ppkBytes = supp.getAsU8Array("account_public_key")

	val ppk = ZcashAccountPubKey(ppkBytes)

    val bytes = ppk.deriveInternalIvk().toBytes()

    assert(ZcashExternalIvk.fromBytes(bytes).toBytes() == bytes)
}
testInternalIvkFromBytes()
