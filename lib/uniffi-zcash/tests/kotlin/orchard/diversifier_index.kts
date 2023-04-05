import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testOrchardDiversifierIndexFromBytes() {
	val expectedBytes = supp.getAsByteArray("orchard_diversifier_index_from_u32")
	val divIdx = ZcashOrchardDiversifierIndex.fromBytes(expectedBytes)

	assert(divIdx.toBytes() == expectedBytes)
}
testOrchardDiversifierIndexFromBytes()

fun testOrchardDiversifierIndexToBytes() {
    // covered by testOrchardDiversifierIndexFromBytes()
}
testOrchardDiversifierIndexToBytes()

fun testOrchardDiversifierIndexFromU32() {
	val integer = supp.getAsU32("orchard_diversifier_index_u32")
	val expectedBytes = supp.getAsByteArray("orchard_diversifier_index_from_u32")

	val divIdx = ZcashOrchardDiversifierIndex.fromU32(integer)

	assert(divIdx.toBytes() == expectedBytes)
}
testOrchardDiversifierIndexFromU32()

fun testOrchardDiversifierIndexFromU64() {
	val integer = supp.getAsU64("orchard_diversifier_index_u64");
	val expectedBytes = supp.getAsByteArray("orchard_diversifier_index_from_u64")

	val divIdx = ZcashOrchardDiversifierIndex.fromU64(integer)

	assert(divIdx.toBytes() == expectedBytes)
}
testOrchardDiversifierIndexFromU64()

