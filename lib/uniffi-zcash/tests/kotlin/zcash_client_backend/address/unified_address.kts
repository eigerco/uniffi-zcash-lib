import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testUnifiedAddressParsing() {
    val seed = supp.getAsByteArray("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val params = ZcashConsensusParameters.MAIN_NETWORK

    var thrown = false;
    try {
        ZcashUnifiedAddress.decode(params, "")
    } catch (e: ZcashException.Message) {
        thrown = true;
    }
    assert(thrown)

    val diversifierBytes = supp.getAsByteArray("diversifier")

    val saplingDiversifier = ZcashDiversifier(diversifierBytes)

    val sapling = unifiedSpendingKey.toUnifiedFullViewingKey()
        .sapling()!!.toIvk(ZcashScope.EXTERNAL)
        .toPaymentAddress(saplingDiversifier)

    val orchardDiversifier = ZcashOrchardDiversifier.fromBytes(diversifierBytes)

    val orchard = unifiedSpendingKey.toUnifiedFullViewingKey()
        .orchard()!!.toIvk(ZcashOrchardScope.EXTERNAL)
        .address(orchardDiversifier)

    val transparentAddressPublicKey = supp.getAsByteArray("transparent_address_public_key")

    val transparent = ZcashTransparentAddress.publicKey(transparentAddressPublicKey)

    val source = ZcashUnifiedAddress(orchard, sapling, transparent)
    val address = source.encode(params)
    val parsed = ZcashUnifiedAddress.decode(params, address)

    assert(address == parsed.encode(params))
}
testUnifiedAddressParsing()

fun testUnifiedAddressCreationWithSapling() {
    val seed = supp.getAsByteArray("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val diversifierBytes = supp.getAsByteArray("diversifier")

    val saplingDiversifier = ZcashDiversifier(diversifierBytes)

    val sapling = unifiedSpendingKey.toUnifiedFullViewingKey()
        .sapling()!!.toIvk(ZcashScope.EXTERNAL)
        .toPaymentAddress(saplingDiversifier)

    var unifiedAddress = ZcashUnifiedAddress(null, sapling, null)

    assert(sapling!!.toBytes() == unifiedAddress.sapling()!!.toBytes())
    assert(null == unifiedAddress.orchard())
}
testUnifiedAddressCreationWithSapling()

fun testUnifiedAddressCreation() {
    val seed = supp.getAsByteArray("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val diversifierBytes = supp.getAsByteArray("diversifier")

    val saplingDiversifier = ZcashDiversifier(diversifierBytes)

    val sapling = unifiedSpendingKey.toUnifiedFullViewingKey()
        .sapling()!!.toIvk(ZcashScope.EXTERNAL)
        .toPaymentAddress(saplingDiversifier)

    val orchardDiversifier = ZcashOrchardDiversifier.fromBytes(diversifierBytes)

    val orchard = unifiedSpendingKey.toUnifiedFullViewingKey()
        .orchard()!!.toIvk(ZcashOrchardScope.EXTERNAL)
        .address(orchardDiversifier)

    val transparentAddressPublicKey = supp.getAsByteArray("transparent_address_public_key")

    val transparent = ZcashTransparentAddress.publicKey(transparentAddressPublicKey)

    // At least one of orchard or sapling address must be set
    // ZcashUnifiedAddress(null, null, null)
    ZcashUnifiedAddress(orchard, null, null)
    ZcashUnifiedAddress(null, sapling, null)
    ZcashUnifiedAddress(orchard, sapling, null)
    // ZcashUnifiedAddress(null, null, transparent)
    ZcashUnifiedAddress(orchard, null, transparent)
    ZcashUnifiedAddress(null, sapling, transparent)
    ZcashUnifiedAddress(orchard, sapling, transparent)
}
testUnifiedAddressCreation()

fun testUnifiedAddressCreationWithOrchard() {
    val seed = supp.getAsByteArray("seed")

    val unifiedSpendingKey = ZcashUnifiedSpendingKey.fromSeed(
        ZcashConsensusParameters.MAIN_NETWORK,
        seed,
        ZcashAccountId(0u),
    )

    val diversifierBytes = supp.getAsByteArray("diversifier")

    val orchardDiversifier = ZcashOrchardDiversifier.fromBytes(diversifierBytes)

    val orchard = unifiedSpendingKey.toUnifiedFullViewingKey()
        .orchard()!!.toIvk(ZcashOrchardScope.EXTERNAL)
        .address(orchardDiversifier)

    val unifiedAddress = ZcashUnifiedAddress(orchard, null, null)

    assert(null == unifiedAddress.sapling())
    assert(orchard.toRawAddressBytes() == unifiedAddress.orchard()!!.toRawAddressBytes())
}
testUnifiedAddressCreationWithOrchard()
