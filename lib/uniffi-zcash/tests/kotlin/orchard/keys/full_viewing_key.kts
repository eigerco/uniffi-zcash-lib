import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun setupGetFvk() =
    ZcashOrchardFullViewingKey.fromBytes(supp.getAsByteArray("orchard_full_viewing_key"))

fun testOrchardFullViewingKeyToBytes() {
    val bytes = supp.getAsByteArray("orchard_full_viewing_key");

    val key = ZcashOrchardFullViewingKey.fromBytes(bytes)

    assert(key.toBytes() == bytes)
}
testOrchardFullViewingKeyToBytes()

fun testOrchardFullViewingKeyFromBytes() {
    // covered by testOrchardFullViewingKeyToBytes()
}
testOrchardFullViewingKeyFromBytes()

fun testOrchardFullViewingKeyAddressAt() {
	val fvk = setupGetFvk()
	val divIdx = ZcashOrchardDiversifierIndex.fromU32(4u)
	val expectedBytes = supp.getAsByteArray("orchard_div_idx_address_at")
	val addr = fvk.addressAt(divIdx, ZcashOrchardScope.EXTERNAL)

	assert(addr.toRawAddressBytes() == expectedBytes)
}
testOrchardFullViewingKeyAddressAt()

fun testOrchardFullViewingKeyAddress() {
	val ofvkBytes = supp.getAsByteArray("orchard_full_viewing_key")
	val fvk = ZcashOrchardFullViewingKey.fromBytes(ofvkBytes)
	val divBytes = supp.getAsByteArray("orchard_diversifier")
	val expectedBytes = supp.getAsByteArray("orchard_div_idx_address")
	val diver = ZcashOrchardDiversifier.fromBytes(divBytes)
	val addr = fvk.address(diver, ZcashOrchardScope.EXTERNAL)

	assert(addr.toRawAddressBytes() == expectedBytes)
}
testOrchardFullViewingKeyAddress()

fun testOrchardFullViewingKeyScopeForAddress() {
	val fvk = setupGetFvk()
	val divBytes = supp.getAsByteArray("orchard_diversifier")
	val diver = ZcashOrchardDiversifier.fromBytes(divBytes)
	val addr = fvk.address(diver, ZcashOrchardScope.EXTERNAL)

	assert(fvk.scopeForAddress(addr) == ZcashOrchardScope.EXTERNAL)
}
testOrchardFullViewingKeyScopeForAddress()

fun testOrchardFullViewingKeyToIvk() {
	val fvk = setupGetFvk()
	val ivk = fvk.toIvk(ZcashOrchardScope.EXTERNAL)
	val expectedBytes = supp.getAsByteArray("orchard_full_viewing_key_ivk")

	assert(ivk.toBytes() == expectedBytes)
}
testOrchardFullViewingKeyToIvk()

fun testOrchardFullViewingKeyToOvk() {
	val fvk = setupGetFvk()
	val ovk = fvk.toOvk(ZcashOrchardScope.EXTERNAL)
	val expectedBytes = supp.getAsByteArray("orchard_full_viewing_key_ovk")

	assert(ovk.toBytes() == expectedBytes)
}
testOrchardFullViewingKeyToOvk()
