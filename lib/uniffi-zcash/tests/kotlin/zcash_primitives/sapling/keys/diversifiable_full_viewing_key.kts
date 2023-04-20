import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testDiversifiableFullViewingKeyFromBytes() {
	val expectedBytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(expectedBytes)

	assert(dfvk.toBytes() == expectedBytes)

}
testDiversifiableFullViewingKeyFromBytes()

fun testDiversifiableFullViewingKeyFvk() {
	val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val fvk = dfvk.fvk()

    val expected = supp.getAsU8Array("diversifiable_fvk_fvk")

	assert(fvk.toBytes() == expected)

}
testDiversifiableFullViewingKeyFvk()

fun testDiversifiableFullViewingKeyToNk() {
	val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

	val nk = dfvk.toNk(ZcashScope.EXTERNAL)

    val expected = supp.getAsU8Array("diversifiable_fvk_nk")

	assert(nk.toBytes() == expected)
}
testDiversifiableFullViewingKeyToNk()

fun testDiversifiableFullViewingKeyToIvk() {
	val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

	val ivk = dfvk.toIvk(ZcashScope.EXTERNAL)

    val expected = supp.getAsU8Array("diversifiable_fvk_ivk")

	assert(ivk.toRepr() == expected)
}
testDiversifiableFullViewingKeyToIvk()

fun testDiversifiableFullViewingKeyToOvk() {
	val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

	val ovk = dfvk.toOvk(ZcashScope.EXTERNAL)

    val expected = supp.getAsU8Array("diversifiable_fvk_ovk")

	assert(ovk.toBytes() == expected)
}
testDiversifiableFullViewingKeyToOvk()

fun testDiversifiableFullViewingKeyAddress() {
	val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val index = ZcashDiversifierIndex.fromU32(1u)

	val address = dfvk.address(index)!!

    val expected = supp.getAsString("diversifiable_fvk_address")

	assert(address.encode(ZcashConsensusParameters.MAIN_NETWORK) == expected)
}
testDiversifiableFullViewingKeyAddress()

fun testDiversifiableFullViewingKeyFindAddress() {
    val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val index = ZcashDiversifierIndex.fromU32(1u)

	val address = dfvk.findAddress(index)!!

    val expectedIndex = supp.getAsU8Array("dfvk_find_address_index")
    val expectedAddress = supp.getAsString("dfvk_find_address_address")

	assert(address.diversifierIndex.toBytes() == expectedIndex)
    assert(address.address.encode(ZcashConsensusParameters.MAIN_NETWORK) == expectedAddress)
}
testDiversifiableFullViewingKeyFindAddress()

fun testDiversifiableFullViewingKeyDefaultAddress() {
    val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val index = ZcashDiversifierIndex.fromU32(1u)

	val address = dfvk.defaultAddress()

    val expectedIndex = supp.getAsU8Array("dfvk_default_address_index")
    val expectedAddress = supp.getAsString("dfvk_default_address_address")

	assert(address.diversifierIndex.toBytes() == expectedIndex)
    assert(address.address.encode(ZcashConsensusParameters.MAIN_NETWORK) == expectedAddress)
}
testDiversifiableFullViewingKeyDefaultAddress()

fun testDiversifiableFullViewingKeyDiversifiedAddress() {
    val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val diversifier = ZcashDiversifier(supp.getAsU8Array("diversifier"))

	val address = dfvk.diversifiedAddress(diversifier)!!

    val expected = supp.getAsString("dfvk_diversified_address")

    assert(address.encode(ZcashConsensusParameters.MAIN_NETWORK) == expected)
}
testDiversifiableFullViewingKeyDiversifiedAddress()

fun testDiversifiableFullViewingKeyChangeAddress() {
    val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val index = ZcashDiversifierIndex.fromU32(1u)

	val address = dfvk.changeAddress()

    val expectedIndex = supp.getAsU8Array("dfvk_change_address_index")
    val expectedAddress = supp.getAsString("dfvk_change_address_address")

	assert(address.diversifierIndex.toBytes() == expectedIndex)
    assert(address.address.encode(ZcashConsensusParameters.MAIN_NETWORK) == expectedAddress)
}
testDiversifiableFullViewingKeyChangeAddress()

fun testDiversifiableFullViewingKeyDiversifiedChangeAddress() {
    val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val diversifier = ZcashDiversifier(supp.getAsU8Array("diversifier"))

	val address = dfvk.diversifiedChangeAddress(diversifier)!!

    val expected = supp.getAsString("dfvk_diversified_change_address")

    assert(address.encode(ZcashConsensusParameters.MAIN_NETWORK) == expected)
}
testDiversifiableFullViewingKeyDiversifiedChangeAddress()

fun testDiversifiableFullViewingKeyDecryptDiversifier() {
    val bytes = supp.getAsU8Array("diversifiable_fvk")

	val dfvk = ZcashDiversifiableFullViewingKey.fromBytes(bytes)

    val address = dfvk.defaultAddress()!!.address

    val decrypted = dfvk.decryptDiversifier(address)!!

    val expected = supp.getAsU8Array("dfvk_decrypt_diversifier")

    assert(decrypted.diversifierIndex.toBytes() == expected)
    assert(decrypted.scope == ZcashScope.EXTERNAL)
}
testDiversifiableFullViewingKeyDecryptDiversifier()
