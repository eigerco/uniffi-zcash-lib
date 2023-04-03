import uniffi.zcash.*

fun testOrchardAddressFromRawAddressBytes() {
	val supp = TestSupport.fromCsvFile()
	val rawBytes = supp.getAsByteArray("orchard_address")
	val zoa = ZcashOrchardAddress.fromRawAddressBytes(rawBytes)

	assert(zoa.toRawAddressBytes() == rawBytes)
}
testOrchardAddressFromRawAddressBytes()

fun testOrchardAddressDiversifier() {
	val supp = TestSupport.fromCsvFile()
	val rawBytes = supp.getAsByteArray("orchard_address")
	val expectedBytes = supp.getAsByteArray("orchard_diversifier_from_bytes")
	val zoa = ZcashOrchardAddress.fromRawAddressBytes(rawBytes)
	val diver = zoa.diversifier().toBytes()

	assert(diver == expectedBytes)
}
testOrchardAddressDiversifier()
