import uniffi.zcash.*


fun testOrchardDiversifierFromBytes() {
	val supp = TestSupport.fromCsvFile()
	val expectedBytes = supp.getAsByteArray("orchard_diversifier_from_bytes")

	val zod = ZcashOrchardDiversifier.fromBytes(expectedBytes)

	assert(expectedBytes == zod.toBytes())
}
testOrchardDiversifierFromBytes()