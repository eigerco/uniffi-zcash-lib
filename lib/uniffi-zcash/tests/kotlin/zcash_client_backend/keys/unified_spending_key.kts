import uniffi.zcash.*

fun testUnifiedSpendingKeyFromSeed() {
    val supp = TestSupport.fromCsvFile()
    val seed = supp.getAsByteArray("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = supp.getAsByteArray("unified_spending_key")

    assert(unifiedSpendingKey.toBytes(ZcashKeysEra.ORCHARD) == expected)
}
testUnifiedSpendingKeyFromSeed()


fun testSpendingKeyConversions() {
    val supp = TestSupport.fromCsvFile()
    val seed = supp.getAsByteArray("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val expectedKeyBytes = supp.getAsByteArray("extended_spending_key")

    assert(key.toBytes() == expectedKeyBytes)
}
testSpendingKeyConversions()

fun testSpendingKeyFromPath(){
    val supp = TestSupport.fromCsvFile()
    val seed = supp.getAsByteArray("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val childIndex = listOf(
            ZcashChildIndex.Hardened(32u),
            ZcashChildIndex.Hardened(133u),
            ZcashChildIndex.Hardened(2u),
            ZcashChildIndex.NonHardened(3u),
    )

    val derivedKey = ZcashExtendedSpendingKey.fromPath(key, childIndex)

    val expectedDerivedKeyBytes = supp.getAsByteArray("extended_spending_key_from_path")
    assert(derivedKey.toBytes() == expectedDerivedKeyBytes)
}
testSpendingKeyFromPath()

fun testSpendingKeyDeriveChild() {
    val supp = TestSupport.fromCsvFile()
    val seed = supp.getAsByteArray("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val derivedKey = key.deriveChild(ZcashChildIndex.Hardened(32u))

    val expectedDerivedKeyBytes = supp.getAsByteArray("extended_spending_key_derived_child")
    assert(derivedKey.toBytes() == expectedDerivedKeyBytes)
}
testSpendingKeyDeriveChild()

fun testSpendingKeyDefaultAddress() {
    val supp = TestSupport.fromCsvFile()
    val seed = supp.getAsByteArray("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val result = key.defaultAddress()

    val expectedAddressBytes = supp.getAsByteArray("extended_spending_key_default_address")

    assert( result.address.toBytes() == expectedAddressBytes)

    val expectedIndexBytes = supp.getAsByteArray("extended_spending_key_child_index")

    assert(result.diversifierIndex.toBytes() == expectedIndexBytes)
}
testSpendingKeyDefaultAddress()

fun testSpendingKeyDeriveInternal(){
    val supp = TestSupport.fromCsvFile()
    val seed = supp.getAsByteArray("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val derivedKey = key.deriveInternal()

    val expectedDerivedKeyBytes = supp.getAsByteArray("extended_spending_key_internal_sk")

    assert(derivedKey.toBytes() == expectedDerivedKeyBytes)
}
testSpendingKeyDeriveInternal()

fun testSpendingKeyToDiversifiableFvk () {
    val supp = TestSupport.fromCsvFile()
    val seed = supp.getAsByteArray("seed")

    val key = ZcashExtendedSpendingKey.master(seed)

    val fvk = key.toDiversifiableFullViewingKey()

    val expectedFvkBytes = supp.getAsByteArray("diversifiable_fvk")

    assert(fvk.toBytes() == expectedFvkBytes)
}
testSpendingKeyToDiversifiableFvk()