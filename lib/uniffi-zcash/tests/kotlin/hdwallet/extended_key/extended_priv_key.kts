import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testExtendedPrivKeyFromRandom() {
	val zepk = ZcashExtendedPrivKey.random()

	// no error thrown

}
testExtendedPrivKeyFromRandom()

fun testExtendedPrivKeyFromRandomWithSeedSize() {
	val seedSize = ZcashKeySeed.S128
	val zepk = ZcashExtendedPrivKey.randomWithSeedSize(seedSize)

	// no errors occurred
}
testExtendedPrivKeyFromRandomWithSeedSize()

fun testExtendedPrivKeyFromRandomWithSeed() {
	val seed = supp.getAsByteArray("seed")
	val zepk = ZcashExtendedPrivKey.withSeed(seed)

	// no errors occurred
}
testExtendedPrivKeyFromRandomWithSeed()

fun testExtendedPrivKeyDerivePrivateKey() {
	val zepk = ZcashExtendedPrivKey.random()
	val idx = ZcashKeyIndex.fromIndex(3u)

	zepk.derivePrivateKey(idx)
}
testExtendedPrivKeyDerivePrivateKey()
