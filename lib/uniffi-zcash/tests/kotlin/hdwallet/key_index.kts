import uniffi.zcash.*

fun testKeyIndexFromU32() {
    val zidx = ZcashKeyIndex.fromIndex(3u)

	// no errors happened
}
testKeyIndexFromU32()

fun testKeyIndexFromIndex() {
	val zidx = ZcashKeyIndex.fromIndex(3u)

	// no errors happened
}
testKeyIndexFromIndex()

fun testKeyIndexRawIndex() {
	val zidx = ZcashKeyIndex.fromIndex(3u)

	assert(zidx.rawIndex() == 3u)
}
testKeyIndexRawIndex()

fun testKeyIndexNormalizeIndex() {
	val zidx = ZcashKeyIndex.fromIndex(2147483648u + 3u)

	assert(zidx.normalizeIndex() == 3u)
}
testKeyIndexNormalizeIndex()

fun testKeyIndexHardenedFromNormalizeIndex() {
	val zidx = ZcashKeyIndex.hardenedFromNormalizeIndex(3u)

    assert(zidx.rawIndex() == 2147483648u + 3u)
	assert(zidx.normalizeIndex() == 3u)
}
testKeyIndexHardenedFromNormalizeIndex()

fun testKeyIndexIsValid() {
	val zidx = ZcashKeyIndex.fromIndex(3u)

	assert(zidx.isValid() == true)
}
testKeyIndexIsValid()
