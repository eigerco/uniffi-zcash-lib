require "test/unit"
require "zcash"

class TestApk < Test::Unit::TestCase
    def test_spending_key_conversions
        zts = Zcash::TestSupport.from_csv_file()

        key_bytes = zts.get_as_byte_array("orchard_spending_key")

        key = Zcash::ZcashOrchardSpendingKey.from_bytes(key_bytes)

        assert_equal(key.to_bytes(), key_bytes)
    end

    def test_spending_key_array_mismatch
        key_bytes = [0, 1].map { |b| [b].pack('c').unpack('c').first }
        assert_raise(Zcash::ZcashError::ArrayLengthMismatch) {  Zcash::ZcashOrchardSpendingKey.from_bytes(key_bytes)}
    end

    def test_spending_key_from_zip32_seed

        zts = Zcash::TestSupport.from_csv_file()

        seed = zts.get_as_byte_array("seed")
        coin_type = zts.get_as_integer("coin_type")
        account = zts.get_as_integer("account")
        expected_bytes = zts.get_as_byte_array("orchard_spending_key_from_zip32_seed")

        key = Zcash::ZcashOrchardSpendingKey.from_zip32_seed(seed, coin_type, account)

        assert_equal(key.to_bytes(), expected_bytes)
    end
end