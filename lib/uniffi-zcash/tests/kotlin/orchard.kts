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

fun testSpendingKeyFromSeed32Seed() {
    val seed = listOf(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0).map { it.toUByte() }
    val coinType = 234u
    val account = 2345u
    
    val keyExpectedBytes = listOf(23, 204, 133, 79, 99, 251, 110, 203, 15, 118, 24, 192, 12, 136, 237, 233, 13, 99, 222, 152, 174, 33, 68, 24, 46, 232, 217, 91, 241, 233, 151, 141).map { it.toUByte() }

    val key = ZcashOrchardSpendingKey.fromZip32Seed(seed, coinType, account)

    assert(key.toBytes() == keyExpectedBytes)
}
testSpendingKeyFromSeed32Seed()