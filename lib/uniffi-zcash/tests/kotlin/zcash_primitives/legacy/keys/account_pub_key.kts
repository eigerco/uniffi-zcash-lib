import uniffi.zcash.*

// TODO some conventions:
 // when it's from_bytes() and when new() ?
 // when is to_bytes present? and as_bytes?

fun testAccountPubKeyNew() {
	val supp = TestSupport.fromCsvFile()
	val ppkBytes = supp.getAsByteArray("account_public_key")

	val ppk = ZcashAccountPubKey.new(ppkBytes)

	assert(ppk.serialize() == ppkBytes)
}
testAccountPubKeyNew()

fun testAccountPubKeyExternalIvk() {
	val supp = TestSupport.fromCsvFile()

	val ppkBytes = supp.getAsByteArray("account_public_key")
	val ivkBytes = supp.getAsByteArray("ppk_external_ivk")

	val ppk = ZcashAccountPubKey.new(ppkBytes)
	val ivk = ppk.deriveExternalIvk()

	assert(ivk.asBytes() == ppkBytes)
}
testAccountPubKeyExternalIvk()

fun testAccountPubKeyInternalIvk() {
	val supp = TestSupport.fromCsvFile()

	val ppkBytes = supp.getAsByteArray("account_public_key")
	val ivkBytes = supp.getAsByteArray("ppk_internal_ivk")

	val ppk = ZcashAccountPubKey.new(ppkBytes)
	val ivk = ppk.deriveInternalIvk()

	assert(ivk.asBytes() == ivkBytes)
}
testAccountPubKeyInternalIvk()

TODO we can cancel this fun I think
fun testAccountPubKeyForShielding() {
	val supp = TestSupport.fromCsvFile()

	val ppkBytes = supp.getAsByteArray("account_public_key")
	val ovkBytes = supp.getAsByteArray("ppk_internal_ovk")

	val ppk = ZcashAccountPubKey.new(ppkBytes)
	val ovks = ppk.ovksForShielding()
	ovks.internal_ovk.a
	ovks.external_ovk

    // TODO ...
}
testAccountPubKeyForShielding()

fun testAccountPubKeyInternalOvk() {
	val supp = TestSupport.fromCsvFile()
	val ppkBytes = supp.getAsByteArray("account_public_key")
	val ovkBytes = supp.getAsByteArray("ppk_internal_ovk")
	val ppk = ZcashAccountPubKey.new(ppkBytes)
	val ovk = ppk.internalOvk()

	assert(ovk.asBytes() == ppkBytes)
}
testAccountPubKeyInternalOvk()

fun testAccountPubKeyExternalOvk() {
	val supp = TestSupport.fromCsvFile()
	val ppkBytes = supp.getAsByteArray("account_public_key")
	val ovkBytes = supp.getAsByteArray("ppk_external_ovk")
	val ppk = ZcashAccountPubKey.new(ppkBytes)
	val ovk = ppk.externalOvk()

	assert(ovk.asBytes() == ppkBytes)
}
testAccountPubKeyExternalOvk()