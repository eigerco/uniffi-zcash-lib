import uniffi.zcash.*

fun testExtendedFullViewingKeyFromBytes() {
	val supp = TestSupport.getFromCsv()

	val fvkBytes = supp.getAsByteArray("extended_spending_key_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	assert(key.toBytes() == fvkBytes)
}
testExtendedFullViewingKeyFromBytes()

fun testExtendedFullViewingKeyEncodeAndDecode() {
	val network = ZcashConsensusParameters.MAIN_NETWORK

	val fvkAddr = supp.getAsByteArray("extended_fvk_encoded")

	val decodedAddr = ZcashExtendedFullViewingKey.decode(network, fvkAddr)

	assert(decodedAddr.encode(network) == fvkAddr)
}
testExtendedFullViewingKeyEncodeAndDecode()

fun testExtendedFullViewingKeyDeriveChild() {
	val supp = TestSupport.getFromCsv()

	val fvkBytes = supp.getAsByteArray("extended_spending_key_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val index = ZcashChildIndex.Hardened(32)

	val efvkChild = key.deriveChild(index)!!.toBytes()

	val fvkChildBytes = supp.getAsByteArray("extended_fvk_child")

	assert(efvkChild.toBytes() == fvkChildBytes)
}
testExtendedFullViewingKeyDeriveChild()

fun testExtendedFullViewingKeyAddress() {
	val supp = TestSupport.getFromCsv()

	val fvkBytes = supp.getAsByteArray("extended_spending_key_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val divIdx = ZcashDiversifierIndex.from_u32(4)

	val paymentAddress = key.address(divIdx)!!.toBytes()

	val fvkAddressBytes = supp.getAsByteArray("extended_fvk_address")

	assert(paymentAddress.toBytes() == fvkAddressBytes)
}
testExtendedFullViewingKeyAddress()

fun testExtendedFullViewingKeyFindAddress() {
	val supp = TestSupport.getFromCsv()

	val fvkBytes = supp.getAsByteArray("extended_spending_key_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val divIdx = ZcashDiversifierIndex.from_u32(0)

	val paymentAddress = key.findAddress(divIdx)

	val expectedBytes = supp.getAsByteArray("extended_fvk_find_address")

	assert(paymentAddress.toBytes() == expectedBytes)
}
testExtendedFullViewingKeyFindAddress()

fun testExtendedFullViewingKeyDefaultAddress() {
	val supp = TestSupport.getFromCsv()

	val fvkBytes = supp.getAsByteArray("extended_spending_key_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val paymentAddress = key.defaultAddress()

	val fvkChildBytes = supp.getAsByteArray("extended_fvk_default_address")

	assert(paymentAddress.toBytes() == paymentAddress)
}
testExtendedFullViewingKeyDefaultAddress()

fun testExtendedFullViewingKeyDeriveInternal() {
	val supp = TestSupport.getFromCsv()

	val fvkBytes = supp.getAsByteArray("extended_spending_key_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val internalEfvk = key.deriveInternal()

	val efvkInternalBytes = supp.getAsByteArray("extended_fvk_derive_internal")

	assert(internalEfvk.toBytes() == efvkInternalBytes)
}
testExtendedFullViewingKeyDeriveInternal()

fun testExtendedFullViewingKeyToDiversifiableFvk() {
	val supp = TestSupport.getFromCsv()

	val fvkBytes = supp.getAsByteArray("extended_spending_key_fvk")

	val key = ZcashExtendedFullViewingKey.fromBytes(fvkBytes)

	val internalEfvk = key.toDiversifiableFullViewingKey()

	val efvkDivBytes = supp.getAsByteArray("extended_fvk_diversifiable_fvk")

	assert(internalEfvk.toBytes() == efvkDivBytes)
}
testExtendedFullViewingKeyToDiversifiableFvk()