import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testAccountPrivKeyFromSeed() {
    val seed = supp.getAsU8Array("seed")

    val accountPrivKey = ZcashAccountPrivKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = supp.getAsU8Array("account_private_key")
    assert(accountPrivKey.toBytes() == expected)
}
testAccountPrivKeyFromSeed()

fun testAccountPrivKeyFromExtendedPrivKey() {
    val seed = supp.getAsU8Array("seed")

    val extendedPrivKey = ZcashExtendedPrivKey.withSeed(seed)
    val accountPrivKey = ZcashAccountPrivKey.fromExtendedPrivkey(extendedPrivKey)

    val expected = supp.getAsU8Array("extended_private_key")
    assert(accountPrivKey.toBytes() == expected)
}
testAccountPrivKeyFromExtendedPrivKey()

fun testAccountPrivKeyToAccountPubKey() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashAccountPrivKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = supp.getAsU8Array("account_public_key")

    assert(key.toAccountPubkey().serialize() == expected)
}
testAccountPrivKeyToAccountPubKey()

fun testAccountPrivKeyDeriveExternalSecretKey() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashAccountPrivKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = supp.getAsU8Array("apk_derive_external_secret_key")

    assert(key.deriveExternalSecretKey(0u).serializeSecret() == expected)
}
testAccountPrivKeyDeriveExternalSecretKey()

fun testAccountPrivKeyDeriveInternalSecretKey() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashAccountPrivKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = supp.getAsU8Array("apk_derive_internal_secret_key")

    assert(key.deriveInternalSecretKey(0u).serializeSecret() == expected)
}
testAccountPrivKeyDeriveInternalSecretKey()

fun testAccountPrivKeyToBytes() {
    // covered by testAccountPrivKeyFromSeed()
}
testAccountPrivKeyToBytes()

fun testAccountPrivKeyFromBytes() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashAccountPrivKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = key.toBytes()

    assert(ZcashAccountPrivKey.fromBytes(expected).toBytes() == expected)
}
testAccountPrivKeyFromBytes()
