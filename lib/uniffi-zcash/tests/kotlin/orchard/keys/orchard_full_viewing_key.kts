import uniffi.zcash.*

// TODO wrong assertion, to investigate
// fun testOrchardFullViewingKeyToFvk() {
// 	val supp = TestSupport.fromCsvFile()
// 	val oskBytes = supp.getAsByteArray("orchard_spending_key")
// 	val ofvkBytes = supp.getAsByteArray("orchard_full_viewing_key")
// 	val fvk = ZcashOrchardFullViewingKey.fromBytes(ofvkBytes)

// 	val osk = ZcashOrchardSpendingKey.fromBytes(oskBytes)
// 	val ofvk = osk.toFvk()

// 	assert(fvk.toBytes() == ofvk.toBytes())
// }
// testOrchardFullViewingKeyToFvk()

fun testOrchardFullViewingKeyAddressAt() {
	val supp = TestSupport.fromCsvFile()
	val ofvkBytes = supp.getAsByteArray("orchard_full_viewing_key")
	val fvk = ZcashOrchardFullViewingKey.fromBytes(ofvkBytes)
	val divIdx = ZcashOrchardDiversifierIndex.fromU32(4u)
	val expectedBytes = supp.getAsByteArray("orchard_div_idx_address_at")
	val addr = fvk.addressAt(divIdx, ZcashOrchardScope.EXTERNAL)

	assert(addr.toRawAddressBytes() == expectedBytes)
}
testOrchardFullViewingKeyAddressAt()

fun testOrchardFullViewingKeyAddress() {
	val supp = TestSupport.fromCsvFile()
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
	val supp = TestSupport.fromCsvFile()
	val ofvkBytes = supp.getAsByteArray("orchard_full_viewing_key")
	val fvk = ZcashOrchardFullViewingKey.fromBytes(ofvkBytes)
	val divBytes = supp.getAsByteArray("orchard_diversifier")
	val diver = ZcashOrchardDiversifier.fromBytes(divBytes)
	val addr = fvk.address(diver, ZcashOrchardScope.EXTERNAL)

	assert(fvk.scopeForAddress(addr) == ZcashOrchardScope.EXTERNAL)
}
testOrchardFullViewingKeyScopeForAddress()

fun testOrchardFullViewingKeyToIvk() {
	val supp = TestSupport.fromCsvFile()
	val ofvkBytes = supp.getAsByteArray("orchard_full_viewing_key")
	val fvk = ZcashOrchardFullViewingKey.fromBytes(ofvkBytes)
	val ivk = fvk.toIvk(ZcashOrchardScope.EXTERNAL)
	val expectedBytes = supp.getAsByteArray("orchard_full_viewing_key_ivk")

	assert(ivk.toBytes() == expectedBytes)
}
testOrchardFullViewingKeyToIvk()

// TODO toBytes

// fun testOrchardFullViewingKeyToOvk() {
// 	val supp = TestSupport.fromCsvFile()
// 	val ofvkBytes = supp.getAsByteArray("orchard_full_viewing_key")
// 	val fvk = ZcashOrchardFullViewingKey.fromBytes(ofvkBytes)
// 	val ovk = fvk.toOvk(ZcashOrchardScope.EXTERNAL)
// 	val expectedBytes = supp.getAsByteArray("orchard_full_viewing_key_ovk")

// 	assert(ovk.toBytes() == expectedBytes)
// }
// testOrchardFullViewingKeyToOvk()
