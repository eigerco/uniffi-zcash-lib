import uniffi.zcash.*

fun setupGetFvk() {
	val supp = TestSupport.getFromCsv()
	val ofvkBytes = supp.getAsByteArray("orchard_full_viewing_key")
	ZcashOrchardFullViewingKey.fromBytes(ofvkBytes)
}

fun testOrchardFullViewingKeyToFvk() {
	val supp = TestSupport.getFromCsv()
	val oskBytes = supp.getAsByteArray("orchard_spending_key")

	val osk = ZcashOrchardSpendingKey.fromBytes(oskBytes)
	val ofvk = osk.toFvk()

	assert(setupGetFvk() == ofvk.toBytes())
}
testOrchardFullViewingKeyToFvk()

fun testOrchardFullViewingKeyAddressAt() {
	val fvk = setupGetFvk()
	val divIdx = ZcashOrchardDiversifierIndex.fromU32(4)
	val expectedBytes = supp.getAsByteArray("orchard_div_idx_address_at")
	val addr = fvk.addressAt(divIdx, ZcashOrchardScope.EXTERNAL)

	assert(addr.toBytes() == expectedBytes)
}
testOrchardFullViewingKeyAddressAt()

fun testOrchardFullViewingKeyAddress() {
	val supp = TestSupport.getFromCsv()
	val ofvkBytes = supp.getAsByteArray("orchard_full_viewing_key")
	val fvk = ZcashOrchardFullViewingKey.fromBytes(ofvkBytes)
	val divBytes = supp.getAsByteArray("orchard_diversifier_from_bytes")
	val expectedBytes = supp.getAsByteArray("orchard_div_idx_address")
	val diver = ZcashOrchardDiversifier.fromBytes(divBytes)
	val addr = fvk.address(diver, ZcashOrchardScope.EXTERNAL)

	assert(addr.toBytes() == expectedBytes)
}
testOrchardFullViewingKeyAddress()

fun testOrchardFullViewingKeyScopeForAddress() {
	val fvk = setupGetFvk()
	val divBytes = supp.getAsByteArray("orchard_diversifier_from_bytes")
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
