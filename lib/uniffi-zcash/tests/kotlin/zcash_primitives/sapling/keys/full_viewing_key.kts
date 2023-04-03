import uniffi.zcash.*

fun testSaplingOvkToBytes() {
	val supp = TestSupport.fromCsvFile()
	val seed = supp.getArgumentAsByteArray("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expected = supp.getAsByteArray("sapling_outgoing_viewing_key")

    assert(unifiedSpendingKey.toUnifiedFullViewingKey()
        .sapling()!!.toOvk(ZcashScope.EXTERNAL)
        .toBytes() == expected)
}
testSaplingOvkToBytes()

fun testOrchardIvkToPaymentAddress() {
	val supp = TestSupport.fromCsvFile()
	val seed = supp.getArgumentAsByteArray("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val orchardDiversifier = ZcashOrchardDiversifier.fromBytes(listOf(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() })

    val expected = listOf(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 118, 188, 133, 2, 30, 187, 222, 192, 24, 118, 136, 143, 249, 3, 127, 83, 48, 137, 67, 228, 146, 86, 27, 251, 163, 42, 159, 247, 98, 150, 25, 7).map { it.toUByte() }

    assert(unifiedSpendingKey.toUnifiedFullViewingKey()
        .orchard()!!.toIvk(ZcashOrchardScope.EXTERNAL)
        .address(orchardDiversifier)!!.toRawAddressBytes() == expected)
}
testOrchardIvkToPaymentAddress()