require "test/unit"
require "zcash"

# Ruby inverts bytes for some reason, so we need to patch it back
# because Uniffi u8 arguments won't take negative bytes
class Array
    def normalized
        self.map { |b| (256 + b) % 256 }
    end

    # bug in uniffi-rs?
    #
    # def readU8
    #  unpack_from 1, 'c'  # I believe it should be 'C' https://apidock.com/ruby/v2_5_5/Array/pack
    # end
    def as_bytes
        self.map { |b| [b].pack('c').unpack('c').first }
    end
end

class TestApk < Test::Unit::TestCase
    def test_spending_key_conversions
        zts = Zcash::TestSupport.from_csv_file()

        key_bytes = zts.get_as_u8_array("orchard_spending_key").normalized

        key = Zcash::ZcashOrchardSpendingKey.from_bytes(key_bytes)

        assert_equal(key.to_bytes().normalized, key_bytes)
    end

    def test_spending_key_array_mismatch
        key_bytes = [0, 1].as_bytes
        assert_raise(Zcash::ZcashError::ArrayLengthMismatch) {  Zcash::ZcashOrchardSpendingKey.from_bytes(key_bytes)}
    end

    def test_spending_key_from_zip32_seed

        zts = Zcash::TestSupport.from_csv_file()

        seed = zts.get_as_u8_array("seed").normalized
        coin_type = zts.get_as_u32("coin_type")
        account = zts.get_as_u32("account")
        key = Zcash::ZcashOrchardSpendingKey.from_zip32_seed(seed, coin_type, account)

        expected_bytes = zts.get_as_u8_array("orchard_spending_key_from_zip32_seed")
        assert_equal(key.to_bytes(), expected_bytes)
    end
end
