import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testSpendingKeyConversions() {
    val keyBytes = supp.getAsByteArray("orchard_spending_key")

    val key = ZcashOrchardSpendingKey.fromBytes(keyBytes)

    assert(key.toBytes() == keyBytes)
}
testSpendingKeyConversions()

fun testSpendingKeyToFvk() {
    val keyBytes = supp.getAsByteArray("orchard_spending_key")

    val key = ZcashOrchardSpendingKey.fromBytes(keyBytes)

    val expectedBytes = supp.getAsByteArray("orchard_spending_key_fvk")

    assert(key.toFvk().toBytes() == expectedBytes)
}
testSpendingKeyToFvk()

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
    val seed = supp.getAsByteArray("seed")
    val coinType = supp.getAsU32("coin_type")
    val account = supp.getAsU32("account")

    val keyExpectedBytes = supp.getAsByteArray("orchard_spending_key_from_zip32_seed")

    val key = ZcashOrchardSpendingKey.fromZip32Seed(seed, coinType, account)

    assert(key.toBytes() == keyExpectedBytes)
}
testSpendingKeyFromZip32Seed()
