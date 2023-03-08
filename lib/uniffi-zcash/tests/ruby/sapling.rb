require "test/unit"
require "zcash"

class TestApk < Test::Unit::TestCase
    def test_spending_key_conversions
        seed = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map { |b| [b].pack('c').unpack('c').first }

        key = Zcash::ZcashExtendedSpendingKey.master(seed)

        key_bytes = key.to_bytes()

        expected_key_bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 195, 198, 202, 45, 72, 50, 2, 237, 36, 207, 134, 196, 143, 180, 241, 151, 208, 35, 90, 48, 36, 29, 242, 225, 171, 4, 154, 32, 23, 74, 135, 254, 60, 215, 251, 107, 26, 246, 149, 17, 106, 197, 95, 43, 176, 26, 114, 78, 147, 230, 102, 117, 217, 219, 121, 221, 141, 243, 105, 242, 196, 17, 6, 125, 154, 91, 206, 197, 239, 46, 185, 26, 182, 28, 221, 75, 161, 188, 104, 15, 86, 66, 21, 75, 78, 129, 45, 4, 99, 26, 58, 74, 86, 119, 1, 123, 105, 66, 67, 174, 51, 49, 192, 41, 58, 102, 153, 105, 232, 85, 21, 192, 24, 52, 203, 7, 85, 50, 219, 6, 97, 47, 52, 118, 47, 147, 0, 215, 4, 12, 231, 93, 214, 87, 86, 214, 95, 91, 215, 83, 54, 134, 176, 145, 16, 19, 163, 192, 7, 116, 107, 102, 91, 195, 249, 245, 41, 46, 70].map { |b| [b].pack('c').unpack('c').first }

        assert_equal(key_bytes, expected_key_bytes)

        Zcash::ZcashExtendedSpendingKey.from_bytes(key_bytes)
    end
    
    def test_spending_key_from_path
        seed = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map { |b| [b].pack('c').unpack('c').first }
        key = Zcash::ZcashExtendedSpendingKey.master(seed)

        child_index = [
            Zcash::ZcashChildIndex::HARDENED.new(32), 
            Zcash::ZcashChildIndex::HARDENED.new(133),
            Zcash::ZcashChildIndex::HARDENED.new(2),
            Zcash::ZcashChildIndex::NON_HARDENED.new(3),
        ]

        derived_key = Zcash::ZcashExtendedSpendingKey.from_path(key, child_index)

        expected_derived_key_bytes = [4, 26, 237, 154, 189, 3, 0, 0, 0, 190, 50, 116, 189, 53, 137, 9, 71, 210, 108, 125, 194, 173, 24, 187, 186, 14, 133, 249, 36, 138, 11, 174, 20, 43, 237, 46, 218, 182, 232, 216, 127, 184, 127, 194, 90, 76, 148, 57, 126, 81, 94, 69, 45, 56, 76, 46, 46, 154, 210, 176, 150, 166, 233, 182, 21, 59, 71, 186, 168, 126, 99, 93, 14, 90, 194, 139, 172, 106, 205, 90, 61, 215, 151, 235, 90, 0, 15, 229, 105, 182, 71, 178, 138, 153, 17, 87, 189, 75, 74, 116, 232, 239, 200, 190, 4, 223, 105, 32, 49, 134, 127, 212, 79, 3, 218, 190, 208, 235, 117, 66, 235, 62, 249, 62, 119, 29, 59, 42, 91, 12, 196, 140, 23, 151, 51, 130, 103, 165, 203, 21, 10, 254, 194, 46, 83, 130, 156, 218, 75, 152, 153, 119, 50, 14, 58, 240, 181, 84, 149, 196, 188, 161, 22, 151, 112, 123, 37, 107, 187].map { |b| [b].pack('c').unpack('c').first }

        assert_equal(derived_key.to_bytes(), expected_derived_key_bytes)
    end
    
    def test_spending_key_derive_child
        seed = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map { |b| [b].pack('c').unpack('c').first }

        key = Zcash::ZcashExtendedSpendingKey.master(seed)

        derived_key = key.derive_child(Zcash::ZcashChildIndex::HARDENED.new(32))

        expected_derived_key_bytes = [1, 218, 182, 42, 238, 32, 0, 0, 128, 15, 138, 2, 193, 121, 32, 219, 17, 134, 25, 134, 93, 158, 90, 28, 34, 125, 195, 175, 185, 30, 223, 112, 158, 65, 160, 216, 168, 76, 86, 1, 219, 19, 65, 107, 114, 227, 231, 62, 196, 207, 83, 181, 21, 198, 185, 156, 211, 235, 215, 133, 233, 176, 98, 210, 232, 233, 32, 140, 24, 128, 153, 38, 9, 97, 77, 211, 193, 69, 175, 211, 71, 79, 1, 123, 27, 72, 49, 64, 180, 55, 202, 198, 55, 64, 72, 77, 134, 29, 114, 210, 223, 11, 63, 210, 1, 53, 73, 172, 144, 203, 7, 79, 239, 215, 40, 231, 131, 33, 202, 175, 158, 66, 216, 167, 222, 67, 54, 218, 137, 175, 235, 191, 193, 103, 178, 60, 96, 91, 201, 140, 62, 175, 124, 173, 170, 68, 4, 129, 95, 45, 189, 39, 214, 40, 1, 222, 82, 243, 17, 102, 236, 197, 198, 215, 198, 209, 187, 83, 220].map { |b| [b].pack('c').unpack('c').first }

        assert_equal(derived_key.to_bytes(), expected_derived_key_bytes)
    end
    
    def test_spending_key_default_address
        seed = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map { |b| [b].pack('c').unpack('c').first }

        key = Zcash::ZcashExtendedSpendingKey.master(seed)

        result = key.default_address()
        
        expected_address_bytes = [91, 59, 79, 228, 216, 68, 167, 156, 199, 182, 182, 184, 107, 245, 37, 145, 194, 241, 226, 63, 157, 130, 209, 140, 137, 229, 45, 115, 56, 194, 31, 118, 140, 33, 179, 60, 74, 226, 114, 199, 101, 216, 161].map { |b| [b].pack('c').unpack('c').first }

        assert_equal(result.address.to_bytes(), expected_address_bytes)

        expected_index_bytes = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map { |b| [b].pack('c').unpack('c').first }

        assert_equal(result.diversifier_index.to_bytes(), expected_index_bytes)
    end    

    def test_spending_key_derive_internal
        seed = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map { |b| [b].pack('c').unpack('c').first }

        key = Zcash::ZcashExtendedSpendingKey.master(seed)

        derived_key = key.derive_internal()
        
        expected_derived_key_bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 195, 198, 202, 45, 72, 50, 2, 237, 36, 207, 134, 196, 143, 180, 241, 151, 208, 35, 90, 48, 36, 29, 242, 225, 171, 4, 154, 32, 23, 74, 135, 254, 60, 215, 251, 107, 26, 246, 149, 17, 106, 197, 95, 43, 176, 26, 114, 78, 147, 230, 102, 117, 217, 219, 121, 221, 141, 243, 105, 242, 196, 17, 6, 121, 209, 143, 180, 33, 137, 166, 142, 214, 103, 176, 171, 151, 188, 142, 87, 90, 50, 78, 79, 228, 154, 68, 183, 17, 91, 125, 90, 236, 27, 42, 6, 23, 159, 15, 53, 108, 230, 232, 135, 19, 31, 121, 130, 26, 233, 11, 243, 222, 23, 184, 46, 212, 75, 194, 186, 225, 111, 54, 239, 211, 251, 61, 87, 134, 175, 157, 85, 96, 14, 187, 153, 221, 181, 4, 120, 22, 17, 65, 203, 204, 206, 159, 46, 1, 19, 73, 177, 78, 243, 121, 114, 129, 220, 59, 205].map { |b| [b].pack('c').unpack('c').first }

        assert_equal(derived_key.to_bytes(), expected_derived_key_bytes)
    end
    
    def test_spending_key_to_divers_fvk
        seed = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map { |b| [b].pack('c').unpack('c').first }

        key = Zcash::ZcashExtendedSpendingKey.master(seed)

        fvk = key.to_diversifiable_full_viewing_key()

        expected_fvk_bytes = [180, 218, 137, 100, 66, 229, 135, 7, 174, 157, 113, 19, 232, 105, 128, 118, 177, 64, 30, 134, 210, 145, 222, 74, 128, 42, 119, 208, 217, 140, 101, 104, 198, 249, 149, 21, 94, 172, 49, 11, 230, 230, 232, 240, 12, 49, 137, 107, 82, 105, 250, 222, 183, 232, 109, 136, 236, 230, 19, 63, 96, 93, 151, 190, 123, 105, 66, 67, 174, 51, 49, 192, 41, 58, 102, 153, 105, 232, 85, 21, 192, 24, 52, 203, 7, 85, 50, 219, 6, 97, 47, 52, 118, 47, 147, 0, 215, 4, 12, 231, 93, 214, 87, 86, 214, 95, 91, 215, 83, 54, 134, 176, 145, 16, 19, 163, 192, 7, 116, 107, 102, 91, 195, 249, 245, 41, 46, 70].map { |b| [b].pack('c').unpack('c').first }

        assert_equal(fvk.to_bytes(), expected_fvk_bytes)
    end

end