import zcash

func testSpendingKeyConversions() {
    let keyBytes: [UInt8] = [166, 3, 186, 151, 20, 139, 99, 33, 212, 134, 101, 192, 119, 208, 167, 21, 119, 228, 7, 152, 74, 140, 84, 209, 236, 235, 53, 57, 109, 65, 44, 178]

    let key = try! ZcashOrchardSpendingKey.fromBytes(data: keyBytes)

    assert(key.toBytes() == keyBytes)
}
testSpendingKeyConversions()

func testSpendingKeyArrayMismatch() {
    let keyBytes: [UInt8] = [0, 1]
    
    var thrown = false;
    do {
        let _ = try ZcashOrchardSpendingKey.fromBytes(data: keyBytes)
    } catch ZcashError.ArrayLengthMismatch {
        thrown = true
    } catch {
    }
    assert(thrown)
}
testSpendingKeyArrayMismatch()

func testSpendingKeyFromSeed32Seed() {
    let seed: [UInt8] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    let coinType: UInt32 = 234
    let account: UInt32 = 2345
    
    let key = try! ZcashOrchardSpendingKey.fromZip32Seed(seed: seed, coinType: coinType, account: account)

    let keyExpectedBytes: [UInt8] = [23, 204, 133, 79, 99, 251, 110, 203, 15, 118, 24, 192, 12, 136, 237, 233, 13, 99, 222, 152, 174, 33, 68, 24, 46, 232, 217, 91, 241, 233, 151, 141]
    assert(key.toBytes() == keyExpectedBytes)
}
testSpendingKeyFromSeed32Seed()