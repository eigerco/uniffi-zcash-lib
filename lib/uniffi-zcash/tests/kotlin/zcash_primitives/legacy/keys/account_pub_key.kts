import uniffi.zcash.*

fun testAccountPubKeyNew() {
	val supp = TestSupport.fromCsvFile()
	val ppkBytes = supp.getAsByteArray("account_public_key")

	val ppk = ZcashAccountPubKey(ppkBytes)

	assert(ppk.serialize() == ppkBytes)
}
testAccountPubKeyNew()

// TODO toBytes

// fun testAccountPubKeyExternalIvk() {
// 	val supp = TestSupport.fromCsvFile()

// 	val ppkBytes = supp.getAsByteArray("account_public_key")
// 	val ivkBytes = supp.getAsByteArray("ppk_external_ivk")

// 	val ppk = ZcashAccountPubKey(ppkBytes)
// 	val ivk = ppk.deriveExternalIvk()

// 	assert(ivk.asBytes() == ppkBytes)
// }
// testAccountPubKeyExternalIvk()

// fun testAccountPubKeyInternalIvk() {
// 	val supp = TestSupport.fromCsvFile()

// 	val ppkBytes = supp.getAsByteArray("account_public_key")
// 	val ivkBytes = supp.getAsByteArray("ppk_internal_ivk")

// 	val ppk = ZcashAccountPubKey(ppkBytes)
// 	val ivk = ppk.deriveInternalIvk()

// 	assert(ivk.asBytes() == ivkBytes)
// }
// testAccountPubKeyInternalIvk()

fun testAccountPubKeyForShielding() {
	val supp = TestSupport.fromCsvFile()

	val ppkBytes = supp.getAsByteArray("account_public_key")
	val intOvkBytes = supp.getAsByteArray("ppk_internal_ovk")
	val extOvkBytes = supp.getAsByteArray("ppk_external_ovk")

	val ppk = ZcashAccountPubKey(ppkBytes)
	val ovks = ppk.ovksForShielding()

	assert(ovks.internalOvk.asBytes() == intOvkBytes)
	assert(ovks.externalOvk.asBytes() == extOvkBytes)
}
testAccountPubKeyForShielding()

fun testAccountPubKeyInternalOvk() {
	val supp = TestSupport.fromCsvFile()
	val ppkBytes = supp.getAsByteArray("account_public_key")
	val ovkBytes = supp.getAsByteArray("ppk_internal_ovk")
	val ppk = ZcashAccountPubKey(ppkBytes)
	val ovk = ppk.internalOvk()

	assert(ovk.asBytes() == ppkBytes)
}
testAccountPubKeyInternalOvk()

fun testAccountPubKeyExternalOvk() {
	val supp = TestSupport.fromCsvFile()
	val ppkBytes = supp.getAsByteArray("account_public_key")
	val ovkBytes = supp.getAsByteArray("ppk_external_ovk")
	val ppk = ZcashAccountPubKey(ppkBytes)
	val ovk = ppk.externalOvk()

	assert(ovk.asBytes() == ppkBytes)
}
testAccountPubKeyExternalOvk()