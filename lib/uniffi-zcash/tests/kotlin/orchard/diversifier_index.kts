import uniffi.zcash.*


// TODO make test intercepting error when: from_u32(too big number)

fun testOrchardDiversifierIndexFromBytes() {
	val supp = TestSupport.fromCsvFile()

	val expectedBytes = supp.getAsByteArray("orchard_diversifier_index_from_u32")
	val divIdx = ZcashOrchardDiversifierIndex.fromBytes(expectedBytes)

	assert(divIdx.toBytes() == expectedBytes)
}
testOrchardDiversifierIndexFromBytes()

fun testOrchardDiversifierIndexFromU32() {
	val supp = TestSupport.fromCsvFile()

	val integer = supp.getAsInteger("orchard_diversifier_index_u32")
	val expectedBytes = supp.getAsByteArray("orchard_diversifier_index_from_u32")

	val divIdx = ZcashOrchardDiversifierIndex.fromU32(integer)

	assert(divIdx.toBytes() == expectedBytes)
}
testOrchardDiversifierIndexFromU32()

fun testOrchardDiversifierIndexFromU64() {
	val supp = TestSupport.fromCsvFile()

	val integer = supp.getAsInteger("orchard_diversifier_index_u64").toULong()
	val expectedBytes = supp.getAsByteArray("orchard_diversifier_index_from_u64")

	val divIdx = ZcashOrchardDiversifierIndex.fromU64(integer)

	assert(divIdx.toBytes() == expectedBytes)
}
testOrchardDiversifierIndexFromU64()

