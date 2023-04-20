import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testDiversifierIndexNew() {
    val index = ZcashDiversifierIndex()

    val expected = supp.getAsU8Array("diversifier_index")

    assert(index.toBytes() == expected)
}
testDiversifierIndexNew()

fun testDiversifierIndexFromU32() {
    val index = ZcashDiversifierIndex.fromU32(0u)

    val expected = supp.getAsU8Array("diversifier_index")

    assert(index.toBytes() == expected)
}
testDiversifierIndexFromU32()

fun testDiversifierIndexFromU64() {
    val index = ZcashDiversifierIndex.fromU32(0U)

    val expected = supp.getAsU8Array("diversifier_index")

    assert(index.toBytes() == expected)
}
testDiversifierIndexFromU64()

fun testDiversifierIndexIncrement() {
    val index = ZcashDiversifierIndex.fromU32(0u)

    index.increment()

    val expected = supp.getAsU8Array("diversifier_index_incremented")

    assert(index.toBytes() == expected)
}
testDiversifierIndexIncrement()

fun testDiversifierIndexToU32() {
    val index = ZcashDiversifierIndex.fromU32(5u)

    assert(index.toU32() == 5u)
}
testDiversifierIndexToU32()
