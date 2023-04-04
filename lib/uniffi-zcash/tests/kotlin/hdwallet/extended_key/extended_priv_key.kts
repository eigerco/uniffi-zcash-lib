import uniffi.zcash.*

fun testExtendedPrivKeyFromRandom() {
	val zepk = ZcashExtendedPrivKey.random()

	assert(false)
	//assert zepk.left() == // no error thrown

}
testExtendedPrivKeyFromRandom()

fun testExtendedPrivKeyFromRandomWithSeedSize() {
	val seedSize = ZcashKeySeed.S128
	val zepk = ZcashExtendedPrivKey.randomWithSeedSize(seedSize)

	assert(false)
	// no errors occurred
}
testExtendedPrivKeyFromRandomWithSeedSize()

fun testExtendedPrivKeyFromRandomWithSeed() {
	val supp = TestSupport.fromCsvFile()
	val seed = supp.getAsByteArray("seed")
	val zepk = ZcashExtendedPrivKey.withSeed(seed)

	assert(false)
	// no errors occurred
}
testExtendedPrivKeyFromRandomWithSeed()

fun testExtendedPrivKeyDerivePrivateKey() {
	val zepk = ZcashExtendedPrivKey.random()
	val idx = ZcashKeyIndex.fromIndex(3u)

	zepk.derivePrivateKey(idx)

	assert(false)

}
testExtendedPrivKeyDerivePrivateKey()
