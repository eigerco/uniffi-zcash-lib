import uniffi.zcash.*

fun testAccountPrivKeyFromSeed() {
    val seed = listOf(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() }

    val accountPrivKey = ZcashAccountPrivKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = listOf(207, 174, 34, 199, 252, 227, 180, 123, 230, 0, 80, 146, 93, 219, 173, 12, 108, 17, 103, 102, 144, 226, 101, 143, 138, 175, 53, 52, 12, 34, 185, 103, 172, 47, 218, 133, 127, 111, 155, 112, 212, 44, 34, 186, 124, 174, 31, 169, 99, 123, 229, 175, 141, 181, 225, 250, 107, 144, 184, 248, 64, 255, 239, 143).map { it.toUByte() }

    assert(accountPrivKey.toBytes() == expected)
}
testAccountPrivKeyFromSeed()

fun testAccountPrivKeyFromExtendedPrivKey() {
    val seed = listOf(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() }

    val extendedPrivKey = ZcashExtendedPrivKey.withSeed(seed)
    val accountPrivKey = ZcashAccountPrivKey.fromExtendedPrivkey(extendedPrivKey)

    val expected = listOf(149, 166, 0, 189, 51, 113, 120, 140, 117, 138, 197, 190, 79, 199, 36, 207, 244, 33, 243, 51, 208, 246, 123, 28, 7, 59, 51, 23, 198, 47, 254, 133, 152, 104, 37, 20, 245, 17, 68, 127, 3, 242, 44, 136, 77, 60, 173, 143, 250, 10, 6, 187, 211, 123, 199, 44, 60, 30, 76, 229, 192, 202, 104, 8).map {it.toUByte()}

    assert(accountPrivKey.toBytes() == expected)
}
testAccountPrivKeyFromExtendedPrivKey()
