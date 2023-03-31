import uniffi.zcash.*

fun testSaplingPaymentAddressParsing() {
    val seed = listOf(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() }

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
    val seed = listOf(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() }

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val saplingDiversifier = ZcashDiversifier(listOf(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() })

    val expected = listOf(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 150, 127, 9, 126, 127, 135, 242, 241, 100, 51, 242, 226, 238, 170, 123, 25, 163, 69, 216, 183, 101, 10, 82, 150, 119, 1, 188, 11, 103, 156, 95).map { it.toUByte() }

    assert(unifiedSpendingKey.toUnifiedFullViewingKey()
        .sapling()!!.toIvk(ZcashScope.EXTERNAL)
        .toPaymentAddress(saplingDiversifier)!!.toBytes() == expected)
}
testSaplingIvkToPaymentAddress()