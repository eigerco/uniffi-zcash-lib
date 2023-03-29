import uniffi.zcash.*

fun testSpendingKeyConversions() {
    val keyBytes = listOf(166, 3, 186, 151, 20, 139, 99, 33, 212, 134, 101, 192, 119, 208, 167, 21, 119, 228, 7, 152, 74, 140, 84, 209, 236, 235, 53, 57, 109, 65, 44, 178).map { it.toUByte() }

    val key = ZcashOrchardSpendingKey.fromBytes(keyBytes)

    assert(key.toBytes() == keyBytes)
}
testSpendingKeyConversions()

fun testSpendingKeyArrayMismatch() {
    val keyBytes = listOf(0, 1).map { it.toUByte() }
    
    var thrown = false;
    try {
        ZcashOrchardSpendingKey.fromBytes(keyBytes)
    } catch (e: ZcashException.ArrayLengthMismatch) {
        thrown = true;
    }
    assert(thrown)
}
testSpendingKeyArrayMismatch()

fun testSpendingKeyFromZip32Seed() {
    val zts = ZcashSpecificTestSupport.fromMethod("SpendingKey::from_zip32_seed")

    val seed = zts.getArgumentAsByteArray(0u)
    val coinType = zts.getArgumentAsInteger(1u)
    val account = zts.getArgumentAsInteger(2u)

    val keyExpectedBytes = zts.getOutputAsBytes()

    val key = ZcashOrchardSpendingKey.fromZip32Seed(seed, coinType, account)

    assert(key.toBytes() == keyExpectedBytes)
}
testSpendingKeyFromZip32Seed()