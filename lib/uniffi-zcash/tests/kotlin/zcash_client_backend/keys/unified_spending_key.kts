import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testUnifiedSpendingKeyFromSeed() {
    val seed = supp.getAsU8Array("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = supp.getAsU8Array("unified_spending_key")

    assert(unifiedSpendingKey.toBytes(ZcashKeysEra.ORCHARD) == expected)
}
testUnifiedSpendingKeyFromSeed()


fun testSpendingKeyConversions() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val expectedKeyBytes = supp.getAsU8Array("extended_spending_key")

    assert(key.toBytes() == expectedKeyBytes)
}
testSpendingKeyConversions()

fun testSpendingKeyFromPath(){
    val seed = supp.getAsU8Array("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val childIndex = listOf(
            ZcashChildIndex.Hardened(32u),
            ZcashChildIndex.Hardened(133u),
            ZcashChildIndex.Hardened(2u),
            ZcashChildIndex.NonHardened(3u),
    )

    val derivedKey = ZcashExtendedSpendingKey.fromPath(key, childIndex)

    val expectedDerivedKeyBytes = supp.getAsU8Array("extended_spending_key_from_path")
    assert(derivedKey.toBytes() == expectedDerivedKeyBytes)
}
testSpendingKeyFromPath()

fun testSpendingKeyDeriveChild() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val derivedKey = key.deriveChild(ZcashChildIndex.Hardened(32u))

    val expectedDerivedKeyBytes = supp.getAsU8Array("extended_spending_key_derived_child")
    assert(derivedKey.toBytes() == expectedDerivedKeyBytes)
}
testSpendingKeyDeriveChild()

fun testSpendingKeyDefaultAddress() {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val result = key.defaultAddress()

    val expectedAddressBytes = supp.getAsU8Array("extended_spending_key_default_address")

    assert( result.address.toBytes() == expectedAddressBytes)

    val expectedIndexBytes = supp.getAsU8Array("extended_spending_key_child_index")

    assert(result.diversifierIndex.toBytes() == expectedIndexBytes)
}
testSpendingKeyDefaultAddress()

fun testSpendingKeyDeriveInternal(){
    val seed = supp.getAsU8Array("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val derivedKey = key.deriveInternal()

    val expectedDerivedKeyBytes = supp.getAsU8Array("extended_spending_key_internal_sk")

    assert(derivedKey.toBytes() == expectedDerivedKeyBytes)
}
testSpendingKeyDeriveInternal()

fun testSpendingKeyToDiversifiableFvk () {
    val seed = supp.getAsU8Array("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val fvk = key.toDiversifiableFullViewingKey()

    val expectedFvkBytes = supp.getAsU8Array("diversifiable_fvk")

    assert(fvk.toBytes() == expectedFvkBytes)
}
testSpendingKeyToDiversifiableFvk()
