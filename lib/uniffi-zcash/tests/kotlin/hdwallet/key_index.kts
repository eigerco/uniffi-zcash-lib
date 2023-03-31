import uniffi.zcash.*

fun testKeyIndexFromIndex() {
	val zidx = ZcashKeyIndex.fromIndex(3)

	// no errors happened
}
testKeyIndexFromIndex()

fun testKeyIndexRawIndex() {
	val zidx = ZcashKeyIndex.fromIndex(3)

	assert(zidx.rawIndex() == 3)
}
testKeyIndexRawIndex()

fun testKeyIndexNormalizeIndex() {
	val zidx = ZcashKeyIndex.fromIndex(3)

	assert(zidx.normalizeIndex() == 300)
}
testKeyIndexNormalizeIndex()

fun testKeyIndexHardenedFromNormalizeIndex() {
	val zidx = ZcashKeyIndex.fromIndex(3)

	// prove that no errors happened

}
testKeyIndexHardenedFromNormalizeIndex()

fun testKeyIndexIsValid() {
	val zidx = ZcashKeyIndex.fromIndex(3)

	assert(zidx.isValid() == true)
}
testKeyIndexIsValid()