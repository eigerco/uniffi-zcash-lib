import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

// (default constructor)
fun testSecpFromSlice() {
    // covered by testSecpSecretKey()
}
testSecpFromSlice()

fun testSecpSecretKey() {
	val expectedBytes = supp.getAsU8Array("secp_secret_key")
	val ssk = SecpSecretKey(expectedBytes)

	assert(ssk.serializeSecret() == expectedBytes)
}
testSecpSecretKey()