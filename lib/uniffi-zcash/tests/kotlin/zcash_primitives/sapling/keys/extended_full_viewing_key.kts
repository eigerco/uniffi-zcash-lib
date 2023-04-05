import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testExtendedFullViewingKeyFromBytes() {
	val fvkBytes = supp.getAsByteArray("extended_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	assert(key.toBytes() == fvkBytes)
}
testExtendedFullViewingKeyFromBytes()

fun testExtendedFullViewingKeyEncodeAndDecode() {
	val network = ZcashConsensusParameters.MAIN_NETWORK

	val fvkAddr = supp.getAsString("extended_fvk_encoded")

	val decodedAddr = ZcashExtendedFullViewingKey.decode(network, fvkAddr)

	assert(decodedAddr.encode(network) == fvkAddr)
}
testExtendedFullViewingKeyEncodeAndDecode()

fun testExtendedFullViewingKeyDeriveChild() {
	val fvkBytes = supp.getAsByteArray("extended_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val index = ZcashChildIndex.Hardened(32u)

	val efvkChild = key.deriveChild(index)

	val fvkChildBytes = supp.getAsByteArray("extended_fvk_child")

	assert(efvkChild.toBytes() == fvkChildBytes)
}
testExtendedFullViewingKeyDeriveChild()

fun testExtendedFullViewingKeyAddress() {
	val fvkBytes = supp.getAsByteArray("extended_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val divIdx = ZcashDiversifierIndex.fromU32(4u)

	val paymentAddress = key.address(divIdx)

	val fvkAddressBytes = supp.getAsByteArray("extended_fvk_address")

	assert(paymentAddress!.toBytes() == fvkAddressBytes)
}
testExtendedFullViewingKeyAddress()

fun testExtendedFullViewingKeyFindAddress() {
	val fvkBytes = supp.getAsByteArray("extended_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val divIdx = ZcashDiversifierIndex.fromU32(0u)

	val paymentAddress = key.findAddress(divIdx)

    val expectedIndexBytes = supp.getAsByteArray("extended_fvk_find_address_index")
	val expectedAddressBytes = supp.getAsByteArray("extended_fvk_find_address_bytes")

    assert(paymentAddress!.diversifierIndex.toBytes() == expectedIndexBytes)
	assert(paymentAddress!.address.toBytes() == expectedAddressBytes)
}
testExtendedFullViewingKeyFindAddress()

fun testExtendedFullViewingKeyDefaultAddress() {
	val fvkBytes = supp.getAsByteArray("extended_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val paymentAddress = key.defaultAddress()

	val fvkChildBytes = supp.getAsByteArray("extended_fvk_default_address")

	assert(paymentAddress.toBytes() == paymentAddress)
}
testExtendedFullViewingKeyDefaultAddress()

fun testExtendedFullViewingKeyDeriveInternal() {
	val fvkBytes = supp.getAsByteArray("extended_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val internalEfvk = key.deriveInternal()

	val efvkInternalBytes = supp.getAsByteArray("extended_fvk_derive_internal")

	assert(internalEfvk.toBytes() == efvkInternalBytes)
}
testExtendedFullViewingKeyDeriveInternal()

fun testExtendedFullViewingKeyToDiversifiableFvk() {
	val fvkBytes = supp.getAsByteArray("extended_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val internalEfvk = key.toDiversifiableFullViewingKey()

	val efvkDivBytes = supp.getAsByteArray("extended_fvk_diversifiable_fvk")

	assert(internalEfvk.toBytes() == efvkDivBytes)
}
testExtendedFullViewingKeyToDiversifiableFvk()
