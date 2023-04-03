import uniffi.zcash.*

fun testAccountPrivKeyFromSeed() {
    val supp = TestSupport.fromCsvFile()
    val seed = supp.getAsByteArray("seed")

    val accountPrivKey = ZcashAccountPrivKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = getAsByteArray("account_private_key")
    assert(accountPrivKey.toBytes() == expected)
}
testAccountPrivKeyFromSeed()

fun testAccountPrivKeyFromExtendedPrivKey() {
    val supp = TestSupport.fromCsvFile()
    val seed = supp.getAsByteArray("seed")

    val extendedPrivKey = ZcashExtendedPrivKey.withSeed(seed)
    val accountPrivKey = ZcashAccountPrivKey.fromExtendedPrivkey(extendedPrivKey)

    val expected = supp.getAsByteArray("extended_private_key")
    assert(accountPrivKey.toBytes() == expected)
}
testAccountPrivKeyFromExtendedPrivKey()
