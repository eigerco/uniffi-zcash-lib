import uniffi.zcash.*

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

// TODO investigate this
// fun testKeyIndexNormalizeIndex() {
// 	val zidx = ZcashKeyIndex.fromIndex(3u)

// 	assert(zidx.normalizeIndex() == 300u)
// }
// testKeyIndexNormalizeIndex()

fun testKeyIndexHardenedFromNormalizeIndex() {
	val zidx = ZcashKeyIndex.fromIndex(3u)

	// prove that no errors happened

}
testKeyIndexHardenedFromNormalizeIndex()

fun testKeyIndexIsValid() {
	val zidx = ZcashKeyIndex.fromIndex(3u)

	assert(zidx.isValid() == true)
}
testKeyIndexIsValid()