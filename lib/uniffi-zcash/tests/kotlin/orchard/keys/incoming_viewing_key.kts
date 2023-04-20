import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testOrchardIncomingViewingKeyToBytes() {
    val bytes = supp.getAsU8Array("orchard_full_viewing_key_ivk");

    val key = ZcashOrchardIncomingViewingKey.fromBytes(bytes)

    assert(key.toBytes() == bytes)
}
testOrchardIncomingViewingKeyToBytes()

fun testOrchardIncomingViewingKeyDiversifierIndex() {
    val bytes = supp.getAsU8Array("orchard_full_viewing_key_ivk");

    val key = ZcashOrchardIncomingViewingKey.fromBytes(bytes)

    val index = ZcashOrchardDiversifierIndex.fromU32(0u)

    val address = key.addressAt(index)

    assert(key.diversifierIndex(address)!!.toBytes() == index.toBytes())
}
testOrchardIncomingViewingKeyDiversifierIndex()

fun testOrchardIncomingViewingKeyAddressAt() {
    val bytes = supp.getAsU8Array("orchard_full_viewing_key_ivk");

    val key = ZcashOrchardIncomingViewingKey.fromBytes(bytes)

    val index = ZcashOrchardDiversifierIndex.fromU32(0u)

    val address = key.addressAt(index)

    val expected = supp.getAsU8Array("orchard_incoming_viewing_key_address_at")

    assert(address.toRawAddressBytes() == expected)
}
testOrchardIncomingViewingKeyAddressAt()

fun testOrchardIncomingViewingKeyAddress() {
    val bytes = supp.getAsU8Array("orchard_full_viewing_key_ivk");

    val key = ZcashOrchardIncomingViewingKey.fromBytes(bytes)

	val zod = ZcashOrchardDiversifier.fromBytes(supp.getAsU8Array("orchard_diversifier"))

    val address = key.address(zod)

    val expected = supp.getAsU8Array("orchard_incoming_viewing_key_address")

    assert(address.toRawAddressBytes() == expected)
}
testOrchardIncomingViewingKeyAddress()
