uniffi::build_foreign_language_testcases!(
    // Kotlin tests
    // UniFFI brings up an entire testing VM per each file,
    // so we keep all tests in one single file for performance reasons.
    // See. https://github.com/mozilla/uniffi-rs/blob/f0be561e98afea00242504f8fdcdef1384574ff1/uniffi_macros/src/test.rs#L11-L43
    "tests/kotlin/big_test.kts",
    // Python tests
    "tests/python/test.py",
    "tests/python/sapling.py",
    "tests/python/orchard.py",
    "tests/python/transaction.py",
    // Ruby tests
    "tests/ruby/test.rb",
    "tests/ruby/sapling.rb",
    "tests/ruby/orchard.rb",
    "tests/ruby/transaction.rb",
    // Swift tests
    "tests/swift/test.swift",
    "tests/swift/sapling.swift",
    "tests/swift/orchard.swift",
    "tests/swift/transaction.swift",
);
