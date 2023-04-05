import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testOrchardIncomingViewingKeyToBytes() {
    val bytes = supp.getAsByteArray("orchard_full_viewing_key_ivk");

    val key = ZcashOrchardIncomingViewingKey.fromBytes(bytes)

    assert(key.toBytes() == bytes)
}
testOrchardIncomingViewingKeyToBytes()

fun testOrchardIncomingViewingKeyFromBytes() {
    // covered by testIncomingViewingKeyToBytes()
}
testOrchardIncomingViewingKeyFromBytes()

fun testOrchardIncomingViewingKeyDiversifierIndex() {
    val bytes = supp.getAsByteArray("orchard_full_viewing_key_ivk");

    val key = ZcashOrchardIncomingViewingKey.fromBytes(bytes)

    val index = ZcashOrchardDiversifierIndex.fromU32(0u)

    val address = key.addressAt(index)

    assert(key.diversifierIndex(address)!!.toBytes() == index.toBytes())
}
testOrchardIncomingViewingKeyDiversifierIndex()

fun testOrchardIncomingViewingKeyAddressAt() {
    val bytes = supp.getAsByteArray("orchard_full_viewing_key_ivk");

    val key = ZcashOrchardIncomingViewingKey.fromBytes(bytes)

    val index = ZcashOrchardDiversifierIndex.fromU32(0u)

    val address = key.addressAt(index)

    val expected = supp.getAsByteArray("orchard_incoming_viewing_key_address_at")

    assert(address.toRawAddressBytes() == expected)
}
testOrchardIncomingViewingKeyAddressAt()

fun testOrchardIncomingViewingKeyAddress() {
    val bytes = supp.getAsByteArray("orchard_full_viewing_key_ivk");

    val key = ZcashOrchardIncomingViewingKey.fromBytes(bytes)

	val zod = ZcashOrchardDiversifier.fromBytes(supp.getAsByteArray("orchard_diversifier"))

    val address = key.address(zod)

    val expected = supp.getAsByteArray("orchard_incoming_viewing_key_address")

    assert(address.toRawAddressBytes() == expected)
}
testOrchardIncomingViewingKeyAddress()
