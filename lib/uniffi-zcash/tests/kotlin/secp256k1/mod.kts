import uniffi.zcash.*

fun testSecpSecretKey() {
	val supp = TestSupport.fromCsvFile()
	val expectedBytes = supp.getAsByteArray("secp_secret_key")
	val ssk = SecpSecretKey.new(expectedBytes)

	assert(ssk.serializeSecret() == expectedBytes)
}
testSecpSecretKey()