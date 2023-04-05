import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testSecpSecretKey() {
	val expectedBytes = supp.getAsByteArray("secp_secret_key")
	val ssk = SecpSecretKey(expectedBytes)

	assert(ssk.serializeSecret() == expectedBytes)
}
testSecpSecretKey()
