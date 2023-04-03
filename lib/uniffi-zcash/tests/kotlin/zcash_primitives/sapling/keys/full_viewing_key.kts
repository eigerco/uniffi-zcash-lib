import uniffi.zcash.*


// TODO the below tests don't belong here
fun testSaplingOvkToBytes() {
	val supp = TestSupport.fromCsvFile()
	val seed = supp.getArgumentAsByteArray("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val expectedBytes = supp.getAsByteArray("sapling_outgoing_viewing_key")

    val ovkBytes = unifiedSpendingKey.toUnifiedFullViewingKey()
        .sapling()!!.toOvk(ZcashScope.EXTERNAL).toBytes()

    assert(ovkBytes == expectedBytes)
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

    val expectedBytes = supp.getAsByteArray("orchard_address")

    val rawAddressBytes = unifiedSpendingKey.toUnifiedFullViewingKey()
        .orchard()!!.toIvk(ZcashOrchardScope.EXTERNAL)
        .address(orchardDiversifier)!!.toRawAddressBytes()

    assert(rawAddressBytes == expectedBytes)
}
testOrchardIvkToPaymentAddress()