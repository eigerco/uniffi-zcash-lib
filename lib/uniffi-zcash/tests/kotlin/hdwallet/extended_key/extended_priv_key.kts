import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testExtendedPrivKeyFromBytes() {
    val bytes = supp.getAsU8Array("hdwallet_epk")

	val zepk = ZcashExtendedPrivKey.fromBytes(bytes)

    assert(zepk.toBytes() == bytes)
}
testExtendedPrivKeyFromBytes()

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

fun testExtendedPrivKeyFromWithSeed() {
	val seed = supp.getAsU8Array("seed")
	val zepk = ZcashExtendedPrivKey.withSeed(seed)

    val bytes = supp.getAsU8Array("hdwallet_epk")

    assert(zepk.toBytes() == bytes)
}
testExtendedPrivKeyFromWithSeed()

fun testExtendedPrivKeyDerivePrivateKey() {
	val seed = supp.getAsU8Array("seed")
	val zepk = ZcashExtendedPrivKey.withSeed(seed)
	val idx = ZcashKeyIndex.fromIndex(3u)

    val expected = supp.getAsU8Array("hdwallet_epk_derive_private_key")

	assert(zepk.derivePrivateKey(idx).toBytes() == expected)
}
testExtendedPrivKeyDerivePrivateKey()
