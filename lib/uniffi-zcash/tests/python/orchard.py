import unittest
from zcash import *

class Test(unittest.TestCase):
    def test_spending_key_conversions(self):
        zts = TestSupport.from_csv_file()
        key_bytes = zts.get_as_u8_array("orchard_spending_key")

        key = ZcashOrchardSpendingKey.from_bytes(key_bytes)

        self.assertEqual(key.to_bytes(), key_bytes)

    def test_spending_key_array_mismatch(self):
        key_bytes = [0, 1]

        with self.assertRaises(ZcashError.ArrayLengthMismatch):
            ZcashOrchardSpendingKey.from_bytes(key_bytes)

    def test_spending_key_from_zip32_seed(self):
        zts = TestSupport.from_csv_file()

        seed = zts.get_as_u8_array("seed")
        coin_type = zts.get_as_u32("coin_type")
        account = zts.get_as_u32("account")

        expected_bytes = zts.get_as_u8_array("orchard_spending_key_from_zip32_seed")

        key = ZcashOrchardSpendingKey.from_zip32_seed(seed, coin_type, account)

        self.assertEqual(key.to_bytes(), expected_bytes)

if __name__ == '__main__':
    unittest.main()
