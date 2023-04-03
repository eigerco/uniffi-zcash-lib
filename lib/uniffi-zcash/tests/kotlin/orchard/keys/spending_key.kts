import uniffi.zcash.*

fun testSpendingKeyConversions() {
    val supp = TestSupport.fromCsvFile()

    val keyBytes = supp.getAsByteArray("orchard_spending_key")

    val key = ZcashOrchardSpendingKey.fromBytes(keyBytes)

    assert(key.toBytes() == keyBytes)
}
testSpendingKeyConversions()

// TODO
fun testSpendingKeyToFVK() {
    val supp = TestSupport.fromCsvFile()

    val keyBytes = supp.getAsByteArray("orchard_spending_key")

    val key = ZcashOrchardSpendingKey.fromBytes(keyBytes)

    assert(key.toBytes() == keyBytes)
}
testSpendingKeyToFVK()

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
    val supp = TestSupport.fromCsvFile()

    val seed = supp.getAsByteArray("seed")
    val coinType = supp.getAsInteger("coin_type")
    val account = supp.getAsInteger("account")

    val keyExpectedBytes = supp.getAsByteArray("orchard_spending_key_from_zip32_seed")

    val key = ZcashOrchardSpendingKey.fromZip32Seed(seed, coinType, account)

    assert(key.toBytes() == keyExpectedBytes)
}
testSpendingKeyFromZip32Seed()