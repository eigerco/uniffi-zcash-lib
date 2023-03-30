import unittest
from zcash import *

class Test(unittest.TestCase):
    def test_spending_key_conversions(self):
        zts = TestSupport.from_csv_file()

        seed = zts.get_as_byte_array("seed")

        key = ZcashExtendedSpendingKey.master(seed)

        key_bytes = key.to_bytes()

        expected_key_bytes = zts.get_as_byte_array("extended_spending_key")

        self.assertEqual(key_bytes, expected_key_bytes)

        ZcashExtendedSpendingKey.from_bytes(key_bytes)
    
    def test_spending_key_from_path(self):
        zts = TestSupport.from_csv_file()

        seed = zts.get_as_byte_array("seed")

        key = ZcashExtendedSpendingKey.master(seed)

        child_index = [
            ZcashChildIndex.HARDENED(32), 
            ZcashChildIndex.HARDENED(133),
            ZcashChildIndex.HARDENED(2),
            ZcashChildIndex.NON_HARDENED(3),
        ]

        derived_key = ZcashExtendedSpendingKey.from_path(key, child_index)

        expected_derived_key_bytes = zts.get_as_byte_array("extended_spending_key_from_path")

        self.assertEqual(derived_key.to_bytes(), expected_derived_key_bytes)
    
    def test_spending_key_derive_child(self):
        zts = TestSupport.from_csv_file()

        seed = zts.get_as_byte_array("seed")

        key = ZcashExtendedSpendingKey.master(seed)

        derived_key = key.derive_child(ZcashChildIndex.HARDENED(32))

        expected_derived_key_bytes = zts.get_as_byte_array("extended_spending_key_derived_child")

        self.assertEqual(derived_key.to_bytes(), expected_derived_key_bytes)
    
    def test_spending_key_default_address(self):
        zts = TestSupport.from_csv_file()

        seed = zts.get_as_byte_array("seed")

        key = ZcashExtendedSpendingKey.master(seed)

        result = key.default_address()
        
        expected_address_bytes = zts.get_as_byte_array("extended_spending_key_default_address")

        self.assertEqual(result.address.to_bytes(), expected_address_bytes)

        expected_index_bytes = zts.get_as_byte_array("extended_spending_key_child_index")

        self.assertEqual(result.diversifier_index.to_bytes(), expected_index_bytes)

    def test_spending_key_derive_internal(self):
        zts = TestSupport.from_csv_file()

        seed = zts.get_as_byte_array("seed")

        key = ZcashExtendedSpendingKey.master(seed)

        derived_key = key.derive_internal()
        
        expected_derived_key_bytes = zts.get_as_byte_array("extended_spending_key_internal_sk")

        self.assertEqual(derived_key.to_bytes(), expected_derived_key_bytes)
    
    def test_spending_key_to_divers_fvk(self):
        zts = TestSupport.from_csv_file()

        seed = zts.get_as_byte_array("seed")

        key = ZcashExtendedSpendingKey.master(seed)

        fvk = key.to_diversifiable_full_viewing_key()

        expected_fvk_bytes = zts.get_as_byte_array("extended_spending_key_fvk")

        self.assertEqual(fvk.to_bytes(), expected_fvk_bytes)

if __name__ == '__main__':
    unittest.main()
