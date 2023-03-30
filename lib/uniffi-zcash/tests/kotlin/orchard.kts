import uniffi.zcash.*

fun testSpendingKeyConversions() {
    val zts = TestSupport.fromCsvFile()

    val keyBytes = zts.getAsByteArray("orchard_spending_key")

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
    val zts = TestSupport.fromCsvFile()

    val seed = zts.getAsByteArray("seed")
    val coinType = zts.getAsInteger("coin_type")
    val account = zts.getAsInteger("account")

    val keyExpectedBytes = zts.getAsByteArray("orchard_spending_key_from_zip32_seed")

    val key = ZcashOrchardSpendingKey.fromZip32Seed(seed, coinType, account)

    assert(key.toBytes() == keyExpectedBytes)
}
testSpendingKeyFromZip32Seed()