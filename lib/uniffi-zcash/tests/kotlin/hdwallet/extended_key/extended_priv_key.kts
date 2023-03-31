import uniffi.zcash.*

fun testExtendedPrivKeyFromRandom() {
	val zepk = ZcashExtendedPrivKey.random()

	assert zepk.left() == // no error thrown

}
testExtendedPrivKeyFromRandom()

fun testExtendedPrivKeyFromRandomWithSeedSize() {
	val seedSize = ZcashKeySeed.S128
	val zepk = ZcashExtendedPrivKey.randomWithSeedSize(seedSize)

	// no errors occurred
}
testExtendedPrivKeyFromRandomWithSeedSize()

fun testExtendedPrivKeyFromRandomWithSeed() {
	val seed = TestSupport.getAsByteArray("seed")
	val zepk = ZcashExtendedPrivKey.withSeed(seed)

	// no errors occurred
}
testExtendedPrivKeyFromRandomWithSeed()

fun testExtendedPrivKeyDerivePrivateKey() {
	val zepk = ZcashExtendedPrivKey.random()
	val idx = ZcashKeyIndex.fromIndex(3)

	zepk.derivePrivateKey(idx)


}
testExtendedPrivKeyDerivePrivateKey()
