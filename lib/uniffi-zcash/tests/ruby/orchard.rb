require "test/unit"
require "zcash"

class TestApk < Test::Unit::TestCase
    def test_spending_key_conversions
        key_bytes = [166, 3, 186, 151, 20, 139, 99, 33, 212, 134, 101, 192, 119, 208, 167, 21, 119, 228, 7, 152, 74, 140, 84, 209, 236, 235, 53, 57, 109, 65, 44, 178].map { |b| [b].pack('c').unpack('c').first } 
        
        key = Zcash::ZcashOrchardSpendingKey.from_bytes(key_bytes)

        assert_equal(key.to_bytes(), key_bytes)
    end

    def test_spending_key_array_mismatch
        key_bytes = [0, 1].map { |b| [b].pack('c').unpack('c').first }
        assert_raise(Zcash::ZcashError::ArrayLengthMismatch) {  Zcash::ZcashOrchardSpendingKey.from_bytes(key_bytes)}
    end

    def test_spending_key_from_zip32_seed
        # seed = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map { |b| [b].pack('c').unpack('c').first }
        # coin_type = 234
        # account = 2345

        zts = Zcash::ZcashSpecificTestSupport.from_method("SpendingKey::from_zip32_seed")
        seed = zts.get_argument_as_byte_array(0).map { |b| [b].pack('c').unpack('c').first }
        coin_type = zts.get_argument_as_integer(1)
        account = zts.get_argument_as_integer(2)
        expected_bytes = zts.get_output_as_bytes()

        key = Zcash::ZcashOrchardSpendingKey.from_zip32_seed(seed, coin_type, account)

        # expected_bytes = [23, 204, 133, 79, 99, 251, 110, 203, 15, 118, 24, 192, 12, 136, 237, 233, 13, 99, 222, 152, 174, 33, 68, 24, 46, 232, 217, 91, 241, 233, 151, 141].map { |b| [b].pack('c').unpack('c').first }

        assert_equal(key.to_bytes(), expected_bytes)
    end
end