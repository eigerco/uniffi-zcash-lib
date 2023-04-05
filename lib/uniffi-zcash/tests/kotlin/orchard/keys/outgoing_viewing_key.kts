import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testOrchardOutgoingViewingKeyFromBytes() {
    val bytes = supp.getAsByteArray("orchard_outgoing_viewing_key");

    val key = ZcashOrchardOutgoingViewingKey.fromBytes(bytes)

    assert(key.toBytes() == bytes)
}
testOrchardOutgoingViewingKeyFromBytes()

fun testOrchardOutgoingViewingKeyToBytes() {
    // covered by testOrchardOutgoingViewingKeyFromBytes()
}
testOrchardOutgoingViewingKeyToBytes()
