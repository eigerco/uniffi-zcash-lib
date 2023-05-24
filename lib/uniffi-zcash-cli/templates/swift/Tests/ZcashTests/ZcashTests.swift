import XCTest
@testable import Zcash

final class ZcashTests: XCTestCase {
    func testAmount() throws {
        let amount = try! ZcashAmount(amount: 200)
        assert(200 == amount.value())
    }
}
