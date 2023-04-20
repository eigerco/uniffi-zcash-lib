import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testOrchardSpendingKeyFromBytes() {
    val keyBytes = supp.getAsU8Array("orchard_spending_key")

    val key = ZcashOrchardSpendingKey.fromBytes(keyBytes)

    assert(key.toBytes() == keyBytes)
}
testOrchardSpendingKeyFromBytes()

fun testOrchardSpendingKeyToFvk() {
    val keyBytes = supp.getAsU8Array("orchard_spending_key")

    val key = ZcashOrchardSpendingKey.fromBytes(keyBytes)

    val expectedBytes = supp.getAsU8Array("orchard_spending_key_fvk")

    assert(key.toFvk().toBytes() == expectedBytes)
}
testOrchardSpendingKeyToFvk()

fun testOrchardSpendingKeyArrayMismatch() {
    val keyBytes = listOf(0, 1).map { it.toUByte() }

    var thrown = false;
    try {
        ZcashOrchardSpendingKey.fromBytes(keyBytes)
    } catch (e: ZcashException.ArrayLengthMismatch) {
        thrown = true;
    }
    assert(thrown)
}
testOrchardSpendingKeyArrayMismatch()

fun testOrchardSpendingKeyFromZip32Seed() {
    val seed = supp.getAsU8Array("seed")
    val coinType = supp.getAsU32("coin_type")
    val account = supp.getAsU32("account")

    val keyExpectedBytes = supp.getAsU8Array("orchard_spending_key_from_zip32_seed")

    val key = ZcashOrchardSpendingKey.fromZip32Seed(seed, coinType, account)

    assert(key.toBytes() == keyExpectedBytes)
}
testOrchardSpendingKeyFromZip32Seed()
