import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()

fun testMemoBytesNew() {
    val bytes = supp.getAsU8Array("memo_bytes")

    val memoData = supp.getAsU8Array("memo_data")

    val memoBytes = ZcashMemoBytes(bytes)

    assert(memoBytes.data() == memoData)

    var thrown = false;
    try {
        val memoBytesTooLong = supp.getAsU8Array("memo_bytes_too_long")
        val memoBytes = ZcashMemoBytes(memoBytesTooLong)
    } catch (e: ZcashException.ArrayLengthMismatch) {
        thrown = true;
    }
    assert(thrown)
}
testMemoBytesNew()

fun testMemoBytesEmpty() {
    val expected = supp.getAsU8Array("memo_empty")

    assert(ZcashMemoBytes.empty().data() == expected)
}
testMemoBytesEmpty()

fun testMemoBytesData() {
    // covered by testMemoBytesNew()
}
testMemoBytesData()
