uniffi::build_foreign_language_testcases!(
    // Python tests
    "tests/python/test.py",
    "tests/python/sapling.py",
    "tests/python/orchard.py",
    "tests/python/transaction.py",
    // Kotlin tests
    "tests/kotlin/test.kts",
    "tests/kotlin/sapling.kts",
    "tests/kotlin/orchard.kts",
    // Ruby tests
    "tests/ruby/test.rb",
    "tests/ruby/sapling.rb",
    "tests/ruby/orchard.rb",
    // Swift tests
    "tests/swift/test.swift",
    "tests/swift/sapling.swift",
    "tests/swift/orchard.swift"
);
