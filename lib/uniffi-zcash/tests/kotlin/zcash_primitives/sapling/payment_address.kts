import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testSaplingPaymentAddressParsing() {
    val seed = supp.getAsByteArray("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val saplingDiversifier = ZcashDiversifier(listOf(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() })

    val sapling = unifiedSpendingKey.toUnifiedFullViewingKey()
        .sapling()!!.toIvk(ZcashScope.EXTERNAL)
        .toPaymentAddress(saplingDiversifier)

    var bytes = sapling!!.toBytes()

    assert(bytes == ZcashPaymentAddress.fromBytes(bytes).toBytes())
}
testSaplingPaymentAddressParsing()

fun testSaplingIvkToPaymentAddress() {
    val seed = supp.getAsByteArray("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val saplingDiversifier = ZcashDiversifier(listOf(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() })

    val expected = supp.getAsByteArray("sapling_address")

    val paymentAddress = unifiedSpendingKey.toUnifiedFullViewingKey()
        .sapling()!!.toIvk(ZcashScope.EXTERNAL)
        .toPaymentAddress(saplingDiversifier)!!.toBytes()

    assert(paymentAddress == expected)
}
testSaplingIvkToPaymentAddress()
