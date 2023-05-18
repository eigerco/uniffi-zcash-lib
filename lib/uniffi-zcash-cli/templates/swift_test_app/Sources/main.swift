// The Swift Programming Language
// https://docs.swift.org/swift-book

import Zcash

let amount = try!  ZcashAmount(amount: 100)
_ = amount.value()
print("Swift test application successfully executed ✅")
