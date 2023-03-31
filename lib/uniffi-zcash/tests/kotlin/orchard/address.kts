import uniffi.zcash.*


fun testOrchardAddressFromRawAddressBytes() {
	val support = TestSupport.fromCsvFile()
	val rawBytes = support.getAsByteArray("orchard_address_bytes")
	val zoa = ZcashOrchardAddress.fromRawAddressBytes(rawBytes)

	assert(zoa.toRawAddressBytes() == rawBytes)

}
testOrchardAddressFromRawAddressBytes()
