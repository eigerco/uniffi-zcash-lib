import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testOrchardDiversifierFromBytes() {
	val expectedBytes = supp.getAsU8Array("orchard_diversifier")

	val zod = ZcashOrchardDiversifier.fromBytes(expectedBytes)

	assert(expectedBytes == zod.toBytes())
}
testOrchardDiversifierFromBytes()

fun testOrchardDiversifierToBytes() {
    // covered by testOrchardDiversifierFromBytes()
}
testOrchardDiversifierToBytes()
