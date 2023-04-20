import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testAccountPubKeyNew() {
	val ppkBytes = supp.getAsU8Array("account_public_key")

	val ppk = ZcashAccountPubKey(ppkBytes)

	assert(ppk.serialize() == ppkBytes)
}
testAccountPubKeyNew()

fun testAccountPubKeyExternalIvk() {
	val ppkBytes = supp.getAsU8Array("account_public_key")
	val ivkBytes = supp.getAsU8Array("ppk_external_ivk")

	val ppk = ZcashAccountPubKey(ppkBytes)
	val ivk = ppk.deriveExternalIvk()

	assert(ivk.toBytes() == ivkBytes)
}
testAccountPubKeyExternalIvk()

fun testAccountPubKeyInternalIvk() {
	val ppkBytes = supp.getAsU8Array("account_public_key")
	val ivkBytes = supp.getAsU8Array("ppk_internal_ivk")

	val ppk = ZcashAccountPubKey(ppkBytes)
	val ivk = ppk.deriveInternalIvk()

	assert(ivk.toBytes() == ivkBytes)
}
testAccountPubKeyInternalIvk()

fun testAccountPubKeyForShielding() {
	val ppkBytes = supp.getAsU8Array("account_public_key")
	val intOvkBytes = supp.getAsU8Array("ppk_internal_ovk")
	val extOvkBytes = supp.getAsU8Array("ppk_external_ovk")

	val ppk = ZcashAccountPubKey(ppkBytes)
	val ovks = ppk.ovksForShielding()

	assert(ovks.internalOvk.asBytes() == intOvkBytes)
	assert(ovks.externalOvk.asBytes() == extOvkBytes)
}
testAccountPubKeyForShielding()

fun testAccountPubKeyInternalOvk() {
	val ppkBytes = supp.getAsU8Array("account_public_key")
	val ovkBytes = supp.getAsU8Array("ppk_internal_ovk")
	val ppk = ZcashAccountPubKey(ppkBytes)
	val ovk = ppk.internalOvk()

	assert(ovk.asBytes() == ovkBytes)
}
testAccountPubKeyInternalOvk()

fun testAccountPubKeyExternalOvk() {
	val ppkBytes = supp.getAsU8Array("account_public_key")
	val ovkBytes = supp.getAsU8Array("ppk_external_ovk")
	val ppk = ZcashAccountPubKey(ppkBytes)
	val ovk = ppk.externalOvk()

	assert(ovk.asBytes() == ovkBytes)
}
testAccountPubKeyExternalOvk()
