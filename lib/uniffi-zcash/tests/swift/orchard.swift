import zcash

func testSpendingKeyConversions() {
    let zts = TestSupport.fromCsvFile()
    let keyBytes: [UInt8] = zts.getAsByteArray(key: "orchard_spending_key")

    let key = try! ZcashOrchardSpendingKey.fromBytes(data: keyBytes)

    assert(key.toBytes() == keyBytes)
}

testSpendingKeyConversions()

func testSpendingKeyArrayMismatch() {
    let keyBytes: [UInt8] = [0, 1]

    var thrown = false
    do {
        _ = try ZcashOrchardSpendingKey.fromBytes(data: keyBytes)
    } catch ZcashError.ArrayLengthMismatch {
        thrown = true
    } catch {}
    assert(thrown)
}

testSpendingKeyArrayMismatch()

func testSpendingKeyFromSeed32Seed() {
    let zts = TestSupport.fromCsvFile()

    let seed: [UInt8] = zts.getAsByteArray(key: "seed")
    let coinType: UInt32 = zts.getAsU32(key: "coin_type")
    let account: UInt32 = zts.getAsU32(key: "account")
    let expectedBytes: [UInt8] = zts.getAsByteArray(key: "orchard_spending_key_from_zip32_seed")

    let key = try! ZcashOrchardSpendingKey.fromZip32Seed(seed: seed, coinType: coinType, account: account)

    assert(key.toBytes() == expectedBytes)
}

testSpendingKeyFromSeed32Seed()
